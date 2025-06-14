//! LLM client module for the LLM-powered project definition wizard.
//!
//! This module handles the communication with the LLM API for generating
//! questions and project definitions.

use anyhow::Result;
use async_openai::{
    types::{ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs, CreateChatCompletionRequest, Role},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Context, Question};
use crate::wizard::question::QuestionType;
use crate::wizard::context::Persona;

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
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_tokens: 2000,
            api_key: None,
        }
    }
}

/// Client for interacting with the LLM API
#[derive(Clone)]
pub struct LlmClient {
    /// The OpenAI client
    client: Client<async_openai::config::OpenAIConfig>,
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
        let client = if let Some(api_key) = &config.api_key {
            let openai_config = async_openai::config::OpenAIConfig::new()
                .with_api_key(api_key);
            Client::with_config(openai_config)
        } else {
            Client::new()
        };

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
    fn create_question_prompt(&self, context: &Context) -> Vec<ChatCompletionRequestMessage> {
        let system_prompt = match context.persona {
            Persona::Default => {
                "You are an intelligent project definition wizard that helps users define LLM-based applications. \
                Generate thoughtful, context-aware questions to understand the user's project requirements. \
                Your questions should build upon previous answers and help create a comprehensive project definition."
            }
            Persona::ProductManager => {
                "You are a Product Manager helping to define an LLM-based application. \
                Ask questions focused on user needs, market fit, success metrics, and product roadmap. \
                Your goal is to ensure the project has clear objectives and delivers value to users."
            }
            Persona::LlmArchitect => {
                "You are an LLM Architect helping to define an LLM-based application. \
                Ask technical questions about model selection, prompt engineering, data requirements, and system architecture. \
                Your goal is to ensure the project is technically feasible and optimally designed."
            }
            Persona::UxDesigner => {
                "You are a UX Designer helping to define an LLM-based application. \
                Ask questions about user experience, interface design, user flows, and accessibility. \
                Your goal is to ensure the project delivers an excellent user experience."
            }
            Persona::ComplianceOfficer => {
                "You are a Compliance Officer helping to define an LLM-based application. \
                Ask questions about data privacy, ethical considerations, regulatory requirements, and risk mitigation. \
                Your goal is to ensure the project complies with relevant regulations and ethical standards."
            }
        };

        let context_str = context.get_context_string();

        let user_prompt = format!(
            "Based on the following context, generate the next question to ask the user about their LLM-based project. \
            The question should help gather more information to create a comprehensive project definition. \
            \n\nCONTEXT:\n{}\n\n\
            Return your response as a JSON object with the following structure:\n\
            {{\n\
              \"question_type\": \"MultipleChoice\" | \"YesNo\" | \"RatingScale\" | \"FreeText\",\n\
              \"question_text\": \"The text of the question\",\n\
              \"options\": [\"Option 1\", \"Option 2\", ...] (only for MultipleChoice),\n\
              \"scale\": [min, max] (only for RatingScale),\n\
              \"help_text\": \"Optional help text for the question\"\n\
            }}\n\
            Make sure the question is relevant to the context and builds upon previous answers.",
            context_str
        );

        vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_prompt)
                .build()
                .unwrap(),
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_prompt)
                .build()
                .unwrap(),
        ]
    }

    /// Create a prompt for generating a project definition
    fn create_project_definition_prompt(&self, context: &Context) -> Vec<ChatCompletionRequestMessage> {
        let system_prompt = "You are an intelligent project definition wizard that helps users define LLM-based applications. \
            Based on the user's answers to your questions, generate a comprehensive project definition document in Markdown format.";

        let context_str = context.get_context_string();

        let user_prompt = format!(
            "Based on the following context, generate a comprehensive project definition document for an LLM-based application. \
            The document should include all the sections mentioned below and be formatted in Markdown.\n\n\
            CONTEXT:\n{}\n\n\
            Include the following sections in the project definition:\n\
            1. Project Name and Short Summary\n\
            2. Use Cases and Goals (with examples or scenarios)\n\
            3. Target User Profile(s)\n\
            4. Required Inputs and Expected Outputs\n\
            5. Functional Components/Modules\n\
            6. Prompt Engineering Strategy\n\
            7. Dataset Needs and Sources\n\
            8. Evaluation Metrics and Success Criteria\n\
            9. Scalability and Deployment Recommendations\n\
            10. Ethical and Bias Considerations\n\n\
            For each section, include a confidence score (1-5) based on the specificity and completeness of the user's answers.",
            context_str
        );

        vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_prompt)
                .build()
                .unwrap(),
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_prompt)
                .build()
                .unwrap(),
        ]
    }

    /// Send a chat request to the LLM API
    async fn send_chat_request(&self, messages: Vec<ChatCompletionRequestMessage>) -> Result<String> {
        let mut request = CreateChatCompletionRequest::default();
        request.model = self.config.model.clone();
        request.messages = messages;
        request.temperature = Some(self.config.temperature);
        request.max_tokens = Some(self.config.max_tokens);

        let response = self.client.chat().create(request).await?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                return Ok(content.clone());
            }
        }

        anyhow::bail!("No response content from LLM")
    }

    /// Parse the LLM response to extract a question
    fn parse_question_response(&self, response: &str) -> Result<Question> {
        // Try to parse the response as JSON
        let parsed: Value = serde_json::from_str(response)
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
