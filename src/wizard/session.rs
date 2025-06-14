//! Session module for the LLM-powered project definition wizard.
//!
//! This module manages the wizard session and coordinates the interaction
//! between the different components.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::{Context, LlmClient, OutputGenerator, Question, QuestionGenerator, Template};

/// State of the wizard session
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SessionState {
    /// Initial state
    Initial,
    /// Collecting user input
    Questioning,
    /// Generating project definition
    Generating,
    /// Session completed
    Completed,
    /// Session error
    Error,
}

/// Session for the wizard
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// Context for the session
    pub context: Context,
    /// Current state of the session
    pub state: SessionState,
    /// Maximum number of questions to ask
    pub max_questions: usize,
    /// Current question
    #[serde(skip)]
    pub current_question: Option<Question>,
    /// Project definition output
    #[serde(skip)]
    pub output: Option<String>,
}

impl Session {
    /// Create a new session
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            state: SessionState::Initial,
            max_questions: 10, // Default max questions
            current_question: None,
            output: None,
        }
    }

    /// Create a new session with a specific context
    pub fn with_context(context: Context) -> Self {
        Self {
            context,
            state: SessionState::Initial,
            max_questions: 10,
            current_question: None,
            output: None,
        }
    }

    /// Create a new session from a template
    pub fn from_template(template: &Template) -> Self {
        let mut context = Context::new();
        template.apply_to_context(&mut context);

        Self {
            context,
            state: SessionState::Initial,
            max_questions: 10,
            current_question: None,
            output: None,
        }
    }

    /// Set the maximum number of questions
    pub fn with_max_questions(mut self, max_questions: usize) -> Self {
        self.max_questions = max_questions;
        self
    }

    /// Save the session to a file
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load a session from a file
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let session: Self = serde_json::from_str(&json)?;
        Ok(session)
    }
}

/// Manager for the wizard session
pub struct SessionManager {
    /// The session being managed
    pub session: Session,
    /// The LLM client
    llm_client: LlmClient,
    /// The question generator
    question_generator: QuestionGenerator,
    /// The output generator
    output_generator: OutputGenerator,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(session: Session, llm_client: LlmClient) -> Self {
        let question_generator = QuestionGenerator::new(llm_client.clone());
        let output_generator = OutputGenerator::new(llm_client.clone());

        Self {
            session,
            llm_client,
            question_generator,
            output_generator,
        }
    }

    /// Start the session
    pub fn start(&mut self) {
        self.session.state = SessionState::Questioning;
    }

    /// Generate the next question
    pub async fn generate_next_question(&mut self) -> Result<&Question> {
        if self.session.state != SessionState::Questioning {
            anyhow::bail!("Session is not in questioning state");
        }

        if self.session.context.history.len() >= self.session.max_questions {
            self.session.state = SessionState::Generating;
            anyhow::bail!("Maximum number of questions reached");
        }

        let question = self.question_generator.generate_next_question(&self.session.context).await?;
        self.session.current_question = Some(question);

        Ok(self.session.current_question.as_ref().unwrap())
    }

    /// Answer the current question
    pub fn answer_question(&mut self, response: impl Into<String>) -> Result<()> {
        if self.session.state != SessionState::Questioning {
            anyhow::bail!("Session is not in questioning state");
        }

        if let Some(question) = self.session.current_question.take() {
            self.session.context.add_answer(question, response);
            Ok(())
        } else {
            anyhow::bail!("No current question to answer");
        }
    }

    /// Go back to a previous question
    pub fn go_back(&mut self) -> Result<&Question> {
        if let Some(answer) = self.session.context.go_back() {
            self.session.current_question = Some(answer.question.clone());
            Ok(&answer.question)
        } else {
            anyhow::bail!("Cannot go back further");
        }
    }

    /// Go forward to a next question (if we've gone back)
    pub fn go_forward(&mut self) -> Result<&Question> {
        if let Some(answer) = self.session.context.go_forward() {
            self.session.current_question = Some(answer.question.clone());
            Ok(&answer.question)
        } else {
            anyhow::bail!("Cannot go forward further");
        }
    }

    /// Generate the project definition
    pub async fn generate_project_definition(&mut self) -> Result<String> {
        self.session.state = SessionState::Generating;

        let project_definition = self.output_generator.generate_project_definition(&self.session.context).await?;
        let markdown = project_definition.to_markdown();

        self.session.output = Some(markdown.clone());
        self.session.state = SessionState::Completed;

        Ok(markdown)
    }

    /// Export the session output to a file
    pub fn export_output(&self, path: impl AsRef<Path>) -> Result<()> {
        if let Some(output) = &self.session.output {
            std::fs::write(path, output)?;
            Ok(())
        } else {
            anyhow::bail!("No output to export");
        }
    }

    /// Get the current question count
    pub fn question_count(&self) -> usize {
        self.session.context.history.len()
    }

    /// Get the maximum number of questions
    pub fn max_questions(&self) -> usize {
        self.session.max_questions
    }

    /// Check if the session is completed
    pub fn is_completed(&self) -> bool {
        self.session.state == SessionState::Completed
    }

    /// Check if the session has an error
    pub fn has_error(&self) -> bool {
        self.session.state == SessionState::Error
    }
}
