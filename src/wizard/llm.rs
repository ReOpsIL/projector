//! LLM client module for the LLM-powered project definition wizard.
//!
//! This module handles the communication with the LLM API for generating
//! questions and project definitions.

use anyhow::Result;
use chrono::{DateTime, Local};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Write;

use super::{Context, Question};
use crate::wizard::context::Persona;
use crate::wizard::question::QuestionType;

/// Configuration for the LLM client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// The model to use for chat completions
    pub model: String,
    /// The temperature parameter for the LLM
    pub temperature: f32,
    /// The maximum number of tokens to generate
    pub max_tokens: u16,
    /// The API key for the LLM service
    pub api_key: Option<String>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model: "google/gemma-3-27b-it:free".to_string(),
            temperature: 1.0,
            max_tokens: 4096,
            api_key: None,
        }
    }
}

/// Role for a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// A message in a chat conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

/// Request for chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u16>,
}

/// Response from chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatCompletionChoice>,
}

/// A choice in a chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub message: ChatMessage,
}

/// Client for interacting with the LLM API
#[derive(Clone)]
pub struct LlmClient {
    /// The HTTP client
    client: reqwest::Client,
    /// Configuration for the LLM
    config: LlmConfig,
}

impl LlmClient {
    /// Create a new LLM client with the default configuration
    pub fn new() -> Result<Self> {
        let config = LlmConfig::default();
        Self::with_config(config)
    }

    /// Create a new LLM client with a custom configuration
    pub fn with_config(config: LlmConfig) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self { client, config })
    }

    /// Generate a question based on the current context
    pub async fn generate_question(&self, context: &Context) -> Result<Question> {
        let prompt = self.create_question_prompt(context);
        let response = self.send_chat_request(prompt).await?;

        // Parse the response to extract the question
        self.parse_question_response(&response)
    }

    /// Generate a project definition based on the context
    pub async fn generate_project_definition(&self, context: &Context) -> Result<String> {
        let prompt = self.create_project_definition_prompt(context);
        let response = self.send_chat_request(prompt).await?;

        Ok(response)
    }

    /// Create a prompt for generating a question
    fn create_question_prompt(&self, context: &Context) -> Vec<ChatMessage> {
        let system_prompt = match context.persona {
            Persona::Default => {
                "You are an intelligent project definition wizard that helps users define applications. \
                Generate thoughtful, context-aware questions to understand the user's project requirements. \
                Your questions should build upon previous answers and help create a comprehensive project definition."
            }
            Persona::ProductManager => {
                "You are a Product Manager helping to define an application. \
                Ask questions focused on user needs, market fit, success metrics, and product roadmap. \
                Your goal is to ensure the project has clear objectives and delivers value to users."
            }
            Persona::LlmArchitect => {
                "You are an software architect helping to define an application. \
                Ask technical questions about model selection, prompt engineering, data requirements, and system architecture. \
                Your goal is to ensure the project is technically feasible and optimally designed."
            }
            Persona::UxDesigner => {
                "You are a UX Designer helping to define an application. \
                Ask questions about user experience, interface design, user flows, and accessibility. \
                Your goal is to ensure the project delivers an excellent user experience."
            }
            Persona::ComplianceOfficer => {
                "You are a Compliance Officer helping to define an application. \
                Ask questions about data privacy, ethical considerations, regulatory requirements, and risk mitigation. \
                Your goal is to ensure the project complies with relevant regulations and ethical standards."
            }
        };

        let persona_name = format!("{:?}", context.persona); // "ProductManager", "UxDesigner", etc.
        let context_str = context.get_context_string();

        let user_prompt = format!(
            r#"Your task is to generate the single best question to ask a user to help define their software project.
                You are acting as a {persona_name}.

                **Rules:**
                1.  Your entire response MUST be a single, valid JSON object. Do not include any explanatory text, markdown formatting, or any characters before or after the JSON.
                2.  The question must logically follow the provided context and aim to fill in missing information.
                3.  If the context is empty or sparse, ask a broad, foundational question.
                4.  If the context is detailed, ask a more specific question to clarify a point or explore a related area.
                5.  Prefer structured questions (`MultipleChoice`, `YesNo`, `RatingScale`) when possible to guide the user, but use `FreeText` for open-ended topics.
                6. Number of answer options should be in range of 4 to 10 
                **Context of the conversation so far:**
                ---
                {context_str}
                ---

                **JSON Output Structure:**
                {{
                  "question_type": "MultipleChoice" | "YesNo" | "RatingScale" | "FreeText",
                  "question_text": "The text of the question to the user.",
                  "options": ["Option 1", "Option 2", ...] // (Required for MultipleChoice, otherwise omit)
                  "scale": [min_number, max_number]     // (Required for RatingScale, otherwise omit)
                  "help_text": "Optional clarifying text for the user." // (Optional, include if the question is complex)
                }}

                **Example:**
                If the user just said "I want to build a chatbot for my website", a good follow-up question would be:

                ```json
                {{
                  "question_type": "MultipleChoice",
                  "question_text": "What is the primary goal of your website chatbot?",
                  "options": [
                    "Answering customer support questions",
                    "Generating sales leads",
                    "Guiding users through the website",
                    .
                    .
                    .
                    "Something else ?"
                  ],
                  "help_text": "This will help us understand the core functionality and success metrics for the chatbot."
                }}
                Now, based on the provided context, generate the next question as a JSON object."#,
            persona_name = persona_name,
            context_str = context_str
        );

        vec![
            ChatMessage {
                role: Role::System,
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: user_prompt,
            },
        ]
    }

    /// Create a prompt for generating a project definition
    fn create_project_definition_prompt(&self, context: &Context) -> Vec<ChatMessage> {
        let system_prompt = "You are an intelligent project definition wizard that helps users define applications. \
            Based on the user's answers to your questions, generate a comprehensive project definition document in Markdown format.";

        let context_str = context.get_context_string();

        let user_prompt = format!(
            r#"Based on the conversation context provided below, generate a comprehensive Project Definition Document.
            **Formatting Instructions:**
            - The entire output must be a single Markdown document.
            - Use `##` for main section titles.
            - Use bullet points (`-`) for lists.
            - Write in a clear, professional, and concise tone suitable for both technical and business stakeholders.

            **Content Instructions:**
            - You must include every section listed below.
            - For each section, provide a **Confidence Score** from 1 (low confidence, very little info) to 5 (high confidence, detailed info).
            - After each confidence score, you MUST provide a brief justification in parentheses. Example: `**Confidence: 2/5** (Reason: The user mentioned a target audience but did not provide specific demographics.)`
            - If you have insufficient information for a section, state that clearly and explain what information is needed.

            **Conversation Context:**
            ---
            {context_str}
            ---

            **Project Definition Document Structure:**

            ## 1. Project Name and Summary
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content...*

            ## 2. Use Cases and Goals
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content... (include specific user scenarios if possible)*

            ## 3. Target User Profile(s)
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content...*

            ## 4. Required Inputs and Expected Outputs
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content... (Detail what the user provides to the system and what the system returns)*

            ## 5. Functional Components/Modules
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content...*

            ## 6. Prompt Engineering Strategy
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content...*

            ## 7. Dataset Needs and Sources
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content...*

            ## 8. Evaluation Metrics and Success Criteria
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content... (How will we know the project is successful?)*

            ## 9. Scalability and Deployment
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content... (Initial thoughts on technical architecture and scaling)*

            ## 10. Ethical and Bias Considerations
            - **Confidence: [1-5]/5** (Reason: ...)
            - *Content...*

            ## 11. Open Questions and Missing Information
            - **Confidence: N/A**
            - *Based on the context, list the key pieces of information that are still needed to complete this project definition.*
            "#,
            context_str = context_str
        );

        vec![
            ChatMessage {
                role: Role::System,
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: user_prompt,
            },
        ]
    }

    /// Send a chat request to the LLM API
    async fn send_chat_request(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            temperature: Some(self.config.temperature),
            max_tokens: Some(self.config.max_tokens),
        };

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(api_key) = &self.config.api_key {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", api_key))?,
            );
        }

        // Send request to OpenRouter API
        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await?;

        if let Some(choice) = response.choices.first() {
            return Ok(choice.message.content.clone());
        }

        anyhow::bail!("No response content from LLM")
    }

    /// Parse the LLM response to extract a question
    fn parse_question_response(&self, response: &str) -> Result<Question> {
        let response = response.trim().replace("```json", "").replace("```", "");
        // Try to parse the response as JSON
        let local_now: DateTime<Local> = Local::now();
        let formatted_local: String = local_now.format("%Y_%m_%d_%H_%M_%S").to_string();

        fs::File::create_new(format!("response_{}.json", formatted_local))
            .unwrap()
            .write_all(response.as_bytes())
            .unwrap();

        let parsed: Value = serde_json::from_str(response.as_str())
            .map_err(|e| anyhow::anyhow!("Failed to parse LLM response as JSON: {}", e))?;

        // Extract the question type
        let question_type = match parsed["question_type"].as_str() {
            Some("MultipleChoice") => QuestionType::MultipleChoice,
            Some("YesNo") => QuestionType::YesNo,
            Some("RatingScale") => QuestionType::RatingScale,
            Some("FreeText") => QuestionType::FreeText,
            _ => anyhow::bail!("Invalid question type in LLM response"),
        };

        // Extract the question text
        let question_text = parsed["question_text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing question_text in LLM response"))?
            .to_string();

        // Generate a unique ID for the question
        let id = format!("q_{}", chrono::Utc::now().timestamp());

        // Create the question based on the type
        let mut question = match question_type {
            QuestionType::MultipleChoice => {
                let options = parsed["options"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Missing options for MultipleChoice question"))?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>();

                Question::multiple_choice(id, question_text, options)
            }
            QuestionType::YesNo => Question::yes_no(id, question_text),
            QuestionType::RatingScale => {
                let scale = parsed["scale"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Missing scale for RatingScale question"))?;

                let min = scale[0]
                    .as_u64()
                    .ok_or_else(|| anyhow::anyhow!("Invalid min value in scale"))?
                    as u8;

                let max = scale[1]
                    .as_u64()
                    .ok_or_else(|| anyhow::anyhow!("Invalid max value in scale"))?
                    as u8;

                Question::rating_scale(id, question_text, min, max)
            }
            QuestionType::FreeText => Question::free_text(id, question_text),
        };

        // Add help text if available
        if let Some(help_text) = parsed["help_text"].as_str() {
            question = question.with_help_text(help_text);
        }

        Ok(question)
    }
}
