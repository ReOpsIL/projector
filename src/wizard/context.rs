//! Context module for the LLM-powered project definition wizard.
//!
//! This module defines the context structure that tracks user responses
//! and maintains the state of the wizard session.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Question;

/// Represents a user's answer to a question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    /// The question that was answered
    pub question: Question,
    /// The user's response
    pub response: String,
    /// Timestamp when the answer was provided
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Answer {
    /// Create a new answer
    pub fn new(question: Question, response: impl Into<String>) -> Self {
        Self {
            question,
            response: response.into(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Enum representing different persona modes for the wizard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Persona {
    /// Default persona
    Default,
    /// Product Manager persona
    ProductManager,
    /// LLM Architect persona
    LlmArchitect,
    /// UX Designer persona
    UxDesigner,
    /// Compliance Officer persona
    ComplianceOfficer,
}

impl Default for Persona {
    fn default() -> Self {
        Self::Default
    }
}

/// Context for the wizard session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// User-provided starting hints
    pub starting_hints: Option<String>,
    /// Selected domain (if any)
    pub domain: Option<String>,
    /// History of questions and answers
    pub history: Vec<Answer>,
    /// Current question index
    pub current_index: usize,
    /// Selected persona mode
    pub persona: Persona,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            starting_hints: None,
            domain: None,
            history: Vec::new(),
            current_index: 0,
            persona: Persona::default(),
            metadata: HashMap::new(),
        }
    }
}

impl Context {
    /// Create a new context
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new context with starting hints
    pub fn with_hints(hints: impl Into<String>) -> Self {
        Self {
            starting_hints: Some(hints.into()),
            ..Self::default()
        }
    }

    /// Create a new context with a specific domain
    pub fn with_domain(domain: impl Into<String>) -> Self {
        Self {
            domain: Some(domain.into()),
            ..Self::default()
        }
    }

    /// Set the persona mode
    pub fn with_persona(mut self, persona: Persona) -> Self {
        self.persona = persona;
        self
    }

    /// Add an answer to the context
    pub fn add_answer(&mut self, question: Question, response: impl Into<String>) {
        let answer = Answer::new(question, response);
        self.history.push(answer);
        self.current_index = self.history.len();
    }

    /// Go back to a previous question
    pub fn go_back(&mut self) -> Option<&Answer> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.history.get(self.current_index)
        } else {
            None
        }
    }

    /// Go forward to a next question (if we've gone back)
    pub fn go_forward(&mut self) -> Option<&Answer> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            self.history.get(self.current_index)
        } else {
            None
        }
    }

    /// Get the current answer
    pub fn current_answer(&self) -> Option<&Answer> {
        if self.current_index < self.history.len() {
            self.history.get(self.current_index)
        } else {
            None
        }
    }

    /// Get all answers as a formatted string for LLM context
    pub fn get_context_string(&self) -> String {
        let mut context = String::new();
        
        // Add starting hints if available
        if let Some(hints) = &self.starting_hints {
            context.push_str(&format!("Starting hints: {}\n\n", hints));
        }
        
        // Add domain if available
        if let Some(domain) = &self.domain {
            context.push_str(&format!("Domain: {}\n\n", domain));
        }
        
        // Add question-answer history
        context.push_str("Previous questions and answers:\n");
        for (i, answer) in self.history.iter().enumerate() {
            context.push_str(&format!(
                "Q{}: {}\nA{}: {}\n\n",
                i + 1,
                answer.question.text,
                i + 1,
                answer.response
            ));
        }
        
        context
    }

    /// Add metadata to the context
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get metadata from the context
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}