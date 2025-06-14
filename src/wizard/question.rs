//! Question module for the LLM-powered project definition wizard.
//!
//! This module defines the different types of questions that can be asked
//! and the logic for generating them based on context.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Enum representing different types of questions that can be asked.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    /// Multiple choice question with options
    MultipleChoice,
    /// Yes/No question
    YesNo,
    /// Rating scale question (e.g., 1-5)
    RatingScale,
    /// Free text question
    FreeText,
}

impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestionType::MultipleChoice => write!(f, "Multiple Choice"),
            QuestionType::YesNo => write!(f, "Yes/No"),
            QuestionType::RatingScale => write!(f, "Rating Scale"),
            QuestionType::FreeText => write!(f, "Free Text"),
        }
    }
}

/// Struct representing a question in the wizard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// The text of the question
    pub text: String,
    /// The type of question
    pub question_type: QuestionType,
    /// Options for multiple choice questions
    pub options: Option<Vec<String>>,
    /// Min and max values for rating scale questions
    pub scale: Option<(u8, u8)>,
    /// Optional help text for the question
    pub help_text: Option<String>,
    /// Unique identifier for the question
    pub id: String,
}

impl Question {
    /// Create a new multiple choice question
    pub fn multiple_choice(
        id: impl Into<String>,
        text: impl Into<String>,
        options: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            question_type: QuestionType::MultipleChoice,
            options: Some(options),
            scale: None,
            help_text: None,
        }
    }

    /// Create a new yes/no question
    pub fn yes_no(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            question_type: QuestionType::YesNo,
            options: Some(vec!["Yes".to_string(), "No".to_string()]),
            scale: None,
            help_text: None,
        }
    }

    /// Create a new rating scale question
    pub fn rating_scale(id: impl Into<String>, text: impl Into<String>, min: u8, max: u8) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            question_type: QuestionType::RatingScale,
            options: None,
            scale: Some((min, max)),
            help_text: None,
        }
    }

    /// Create a new free text question
    pub fn free_text(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            question_type: QuestionType::FreeText,
            options: None,
            scale: None,
            help_text: None,
        }
    }

    /// Add help text to the question
    pub fn with_help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }
}

/// Struct for generating questions based on context
pub struct QuestionGenerator {
    /// The LLM client used for generating questions
    llm_client: crate::wizard::LlmClient,
}

impl QuestionGenerator {
    /// Create a new question generator
    pub fn new(llm_client: crate::wizard::LlmClient) -> Self {
        Self { llm_client }
    }

    /// Generate the next question based on the current context
    pub async fn generate_next_question(
        &self,
        context: &crate::wizard::Context,
    ) -> anyhow::Result<Question> {
        // Use the LLM to generate the next question based on the context
        self.llm_client.generate_question(context).await
    }
}
