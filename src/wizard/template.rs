//! Template module for the LLM-powered project definition wizard.
//!
//! This module provides predefined templates and presets for different
//! types of LLM-based applications.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::{Config, Context, Question};

pub(crate) type Domain = String;

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
    /// Configuration
    config: Config,
}

impl TemplateRepository {
    /// Create a new template repository with default templates
    pub fn new() -> Self {
        // Try to load configuration from default path
        let config = Self::load_default_config().unwrap_or_else(|_| Config::default());

        let mut repo = Self {
            templates: Vec::new(),
            config,
        };

        repo
    }

    /// Create a new template repository with configuration from a specific path
    pub fn with_config<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config = Config::load_from_file(config_path)?;

        let mut repo = Self {
            templates: Vec::new(),
            config,
        };

        Ok(repo)
    }

    /// Load configuration from the default path
    fn load_default_config() -> Result<Config> {
        let default_path = Config::default_path();
        if default_path.exists() {
            Config::load_from_file(default_path)
        } else {
            Ok(Config::default())
        }
    }

    /// Get all available domains
    pub fn get_all_domains(&self) -> Vec<Domain> {
        self.config.domains.clone()
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
                (a, b) => a == b,
            })
            .collect()
    }
}
