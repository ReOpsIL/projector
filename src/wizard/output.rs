//! Output generator module for the LLM-powered project definition wizard.
//!
//! This module handles the generation of the final project definition document
//! in Markdown format.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::{Context, LlmClient};

/// Confidence level for sections of the project definition
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    /// Very low confidence (1/5)
    VeryLow = 1,
    /// Low confidence (2/5)
    Low = 2,
    /// Medium confidence (3/5)
    Medium = 3,
    /// High confidence (4/5)
    High = 4,
    /// Very high confidence (5/5)
    VeryHigh = 5,
}

impl ConfidenceLevel {
    /// Convert a numeric value to a confidence level
    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::VeryLow),
            2 => Some(Self::Low),
            3 => Some(Self::Medium),
            4 => Some(Self::High),
            5 => Some(Self::VeryHigh),
            _ => None,
        }
    }

    /// Get the emoji representation of the confidence level
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::VeryLow => "⚠️",
            Self::Low => "🔸",
            Self::Medium => "🔶",
            Self::High => "✅",
            Self::VeryHigh => "⭐",
        }
    }
}

/// Section of the project definition document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSection {
    /// Title of the section
    pub title: String,
    /// Content of the section
    pub content: String,
    /// Confidence level for the section
    pub confidence: ConfidenceLevel,
}

/// Complete project definition document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDefinition {
    /// Project name
    pub name: String,
    /// Sections of the project definition
    pub sections: Vec<ProjectSection>,
    /// Timestamp when the definition was generated
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ProjectDefinition {
    /// Create a new project definition
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sections: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Add a section to the project definition
    pub fn add_section(
        &mut self,
        title: impl Into<String>,
        content: impl Into<String>,
        confidence: ConfidenceLevel,
    ) {
        self.sections.push(ProjectSection {
            title: title.into(),
            content: content.into(),
            confidence,
        });
    }

    /// Convert the project definition to a Markdown string
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        // Add title
        markdown.push_str(&format!("# {}\n\n", self.name));

        // Add timestamp
        markdown.push_str(&format!(
            "*Generated on: {}*\n\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Add sections
        for section in &self.sections {
            markdown.push_str(&format!(
                "## {} {}\n\n",
                section.title,
                section.confidence.emoji()
            ));
            markdown.push_str(&format!("{}\n\n", section.content));
        }

        markdown
    }

    /// Save the project definition to a file
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let markdown = self.to_markdown();
        fs::write(path, markdown)?;
        Ok(())
    }
}

/// Generator for project definition documents
pub struct OutputGenerator {
    /// The LLM client used for generating project definitions
    llm_client: LlmClient,
}

impl OutputGenerator {
    /// Create a new output generator
    pub fn new(llm_client: LlmClient) -> Self {
        Self { llm_client }
    }

    /// Generate a project definition from the context
    pub async fn generate_project_definition(
        &self,
        context: &Context,
    ) -> Result<ProjectDefinition> {
        // Use the LLM to generate the project definition
        let markdown = self.llm_client.generate_project_definition(context).await?;

        // Parse the markdown to extract sections and confidence levels
        self.parse_markdown_definition(&markdown)
    }

    /// Parse the markdown project definition to extract sections and confidence levels
    fn parse_markdown_definition(&self, markdown: &str) -> Result<ProjectDefinition> {
        // Extract the project name from the first heading
        let lines: Vec<&str> = markdown.lines().collect();
        let project_name = lines
            .iter()
            .find(|line| line.starts_with("# "))
            .map(|line| line[2..].trim().to_string())
            .unwrap_or_else(|| "LLM Project Definition".to_string());

        let mut definition = ProjectDefinition::new(project_name);

        // Extract sections
        let mut current_section_title = String::new();
        let mut current_section_content = String::new();
        let mut current_confidence = ConfidenceLevel::Medium;

        for line in lines {
            if line.starts_with("## ") {
                // Save the previous section if it exists
                if !current_section_title.is_empty() && !current_section_content.is_empty() {
                    definition.add_section(
                        current_section_title,
                        current_section_content,
                        current_confidence,
                    );
                    current_section_content = String::new();
                }

                // Parse the new section title and confidence
                let title_line = line[3..].trim();

                // Extract confidence from emojis or explicit markers
                current_confidence = if title_line.contains("⭐") {
                    ConfidenceLevel::VeryHigh
                } else if title_line.contains("✅") {
                    ConfidenceLevel::High
                } else if title_line.contains("🔶") {
                    ConfidenceLevel::Medium
                } else if title_line.contains("🔸") {
                    ConfidenceLevel::Low
                } else if title_line.contains("⚠️") {
                    ConfidenceLevel::VeryLow
                } else if title_line.contains("(Confidence: 5/5)") {
                    ConfidenceLevel::VeryHigh
                } else if title_line.contains("(Confidence: 4/5)") {
                    ConfidenceLevel::High
                } else if title_line.contains("(Confidence: 3/5)") {
                    ConfidenceLevel::Medium
                } else if title_line.contains("(Confidence: 2/5)") {
                    ConfidenceLevel::Low
                } else if title_line.contains("(Confidence: 1/5)") {
                    ConfidenceLevel::VeryLow
                } else {
                    ConfidenceLevel::Medium // Default
                };

                // Clean the title by removing confidence markers
                current_section_title = title_line
                    .replace("⭐", "")
                    .replace("✅", "")
                    .replace("🔶", "")
                    .replace("🔸", "")
                    .replace("⚠️", "")
                    .replace("(Confidence: 5/5)", "")
                    .replace("(Confidence: 4/5)", "")
                    .replace("(Confidence: 3/5)", "")
                    .replace("(Confidence: 2/5)", "")
                    .replace("(Confidence: 1/5)", "")
                    .trim()
                    .to_string();
            } else if !current_section_title.is_empty() {
                // Add the line to the current section content
                current_section_content.push_str(line);
                current_section_content.push('\n');
            }
        }

        // Add the last section if it exists
        if !current_section_title.is_empty() && !current_section_content.is_empty() {
            definition.add_section(
                current_section_title,
                current_section_content,
                current_confidence,
            );
        }

        Ok(definition)
    }
}
