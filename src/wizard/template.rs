//! Template module for the LLM-powered project definition wizard.
//!
//! This module provides predefined templates and presets for different
//! types of LLM-based applications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Context, Question};
use crate::wizard::context::Persona;
use crate::wizard::question::QuestionType;

/// Domain for the LLM-based application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Domain {
    /// Legal domain
    Legal,
    /// Medical domain
    Medical,
    /// Software development domain
    SoftwareDevelopment,
    /// Education domain
    Education,
    /// Customer service domain
    CustomerService,
    /// Finance domain
    Finance,
    /// Custom domain
    Custom(String),
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legal => write!(f, "Legal"),
            Self::Medical => write!(f, "Medical"),
            Self::SoftwareDevelopment => write!(f, "Software Development"),
            Self::Education => write!(f, "Education"),
            Self::CustomerService => write!(f, "Customer Service"),
            Self::Finance => write!(f, "Finance"),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Template for an LLM-based application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Name of the template
    pub name: String,
    /// Description of the template
    pub description: String,
    /// Domain of the template
    pub domain: Domain,
    /// Starting hints for the template
    pub starting_hints: String,
    /// Initial questions for the template
    pub initial_questions: Vec<Question>,
    /// Metadata for the template
    pub metadata: HashMap<String, String>,
}

impl Template {
    /// Create a new template
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        domain: Domain,
        starting_hints: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            domain,
            starting_hints: starting_hints.into(),
            initial_questions: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add an initial question to the template
    pub fn add_question(&mut self, question: Question) {
        self.initial_questions.push(question);
    }

    /// Add metadata to the template
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Apply the template to a context
    pub fn apply_to_context(&self, context: &mut Context) {
        // Set the starting hints
        context.starting_hints = Some(self.starting_hints.clone());

        // Set the domain
        context.domain = Some(self.domain.to_string());

        // Add metadata
        for (key, value) in &self.metadata {
            context.add_metadata(key, value);
        }
    }
}

/// Repository of templates
pub struct TemplateRepository {
    /// Available templates
    templates: Vec<Template>,
}

impl TemplateRepository {
    /// Create a new template repository with default templates
    pub fn new() -> Self {
        let mut repo = Self {
            templates: Vec::new(),
        };

        repo.add_default_templates();

        repo
    }

    /// Add a template to the repository
    pub fn add_template(&mut self, template: Template) {
        self.templates.push(template);
    }

    /// Get a template by name
    pub fn get_template(&self, name: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.name == name)
    }

    /// Get all templates
    pub fn get_all_templates(&self) -> &[Template] {
        &self.templates
    }

    /// Get templates by domain
    pub fn get_templates_by_domain(&self, domain: &Domain) -> Vec<&Template> {
        self.templates
            .iter()
            .filter(|t| match (&t.domain, domain) {
                (Domain::Custom(a), Domain::Custom(b)) => a == b,
                (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
            })
            .collect()
    }

    /// Add default templates to the repository
    fn add_default_templates(&mut self) {
        // Legal Assistant Template
        let mut legal_assistant = Template::new(
            "Legal Assistant",
            "An AI assistant that helps with legal research, document analysis, and case preparation",
            Domain::Legal,
            "I want to build an LLM-based legal assistant that can help lawyers with research and document analysis",
        );

        legal_assistant.add_question(Question::multiple_choice(
            "legal_focus",
            "What is the primary focus of your legal assistant?",
            vec![
                "Legal research and case law analysis".to_string(),
                "Contract review and analysis".to_string(),
                "Legal document drafting".to_string(),
                "Compliance monitoring".to_string(),
                "Client intake and case management".to_string(),
            ],
        ));

        legal_assistant.add_question(Question::yes_no(
            "confidentiality",
            "Will your legal assistant need to handle confidential or privileged information?",
        ));

        legal_assistant.add_metadata("industry", "legal");
        legal_assistant.add_metadata("security_level", "high");

        self.add_template(legal_assistant);

        // Medical Chatbot Template
        let mut medical_chatbot = Template::new(
            "Medical Chatbot",
            "An AI assistant that provides medical information, symptom checking, and healthcare guidance",
            Domain::Medical,
            "I want to build an LLM-based medical chatbot that can provide healthcare information and guidance",
        );

        medical_chatbot.add_question(Question::multiple_choice(
            "medical_focus",
            "What is the primary focus of your medical chatbot?",
            vec![
                "Patient education and information".to_string(),
                "Symptom checking and triage".to_string(),
                "Medication reminders and adherence".to_string(),
                "Mental health support".to_string(),
                "Healthcare provider support".to_string(),
            ],
        ));

        medical_chatbot.add_question(Question::rating_scale(
            "medical_expertise",
            "How specialized should the medical knowledge be? (1: General health, 5: Highly specialized)",
            1,
            5,
        ));

        medical_chatbot.add_metadata("industry", "healthcare");
        medical_chatbot.add_metadata("regulatory", "HIPAA");

        self.add_template(medical_chatbot);

        // Code Explainer Template
        let mut code_explainer = Template::new(
            "Code Explainer",
            "An AI assistant that explains code, suggests improvements, and helps with programming tasks",
            Domain::SoftwareDevelopment,
            "I want to build an LLM-based code explainer that can help developers understand and improve their code",
        );

        code_explainer.add_question(Question::multiple_choice(
            "programming_languages",
            "Which programming languages should your code explainer focus on?",
            vec![
                "Python".to_string(),
                "JavaScript/TypeScript".to_string(),
                "Java".to_string(),
                "C/C++".to_string(),
                "Rust".to_string(),
                "Go".to_string(),
                "Multiple languages".to_string(),
            ],
        ));

        code_explainer.add_question(Question::free_text(
            "target_users",
            "Who are the target users of your code explainer? (e.g., beginners, professionals, students)",
        ));

        code_explainer.add_metadata("industry", "software");
        code_explainer.add_metadata("technical_level", "high");

        self.add_template(code_explainer);

        // Educational Tutor Template
        let mut educational_tutor = Template::new(
            "Educational Tutor",
            "An AI tutor that helps students learn subjects, answer questions, and prepare for exams",
            Domain::Education,
            "I want to build an LLM-based educational tutor that can help students learn and understand various subjects",
        );

        educational_tutor.add_question(Question::multiple_choice(
            "subject_area",
            "What subject area should your educational tutor focus on?",
            vec![
                "Mathematics".to_string(),
                "Science".to_string(),
                "History".to_string(),
                "Literature".to_string(),
                "Computer Science".to_string(),
                "Languages".to_string(),
                "Multiple subjects".to_string(),
            ],
        ));

        educational_tutor.add_question(Question::multiple_choice(
            "education_level",
            "What education level should your tutor target?",
            vec![
                "Elementary school".to_string(),
                "Middle school".to_string(),
                "High school".to_string(),
                "Undergraduate".to_string(),
                "Graduate".to_string(),
                "Professional".to_string(),
                "Multiple levels".to_string(),
            ],
        ));

        educational_tutor.add_metadata("industry", "education");
        educational_tutor.add_metadata("audience", "students");

        self.add_template(educational_tutor);
    }
}
