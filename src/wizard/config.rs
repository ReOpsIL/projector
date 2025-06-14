//! Configuration module for the LLM-powered project definition wizard.
//!
//! This module handles loading and managing configuration settings,
//! including domain definitions.

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// Configuration for the wizard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Available domains
    pub domains: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        // Default configuration with 100 random domains
        let default_domains = vec![
            "Accounting",
            "Advertising",
            "Aerospace",
            "Agriculture",
            "AI Research",
            "Architecture",
            "Art",
            "Automotive",
            "Banking",
            "Biotechnology",
            "Blockchain",
            "Chemistry",
            "Childcare",
            "Cinema",
            "Civil Engineering",
            "Climate Science",
            "Cloud Computing",
            "Construction",
            "Consulting",
            "Cosmetics",
            "Cryptocurrency",
            "Cybersecurity",
            "Data Analysis",
            "Defense",
            "Design",
            "E-commerce",
            "Economics",
            "Electrical Engineering",
            "Electronics",
            "Energy",
            "Entertainment",
            "Environmental Science",
            "Event Management",
            "Fashion",
            "Film Production",
            "Financial Services",
            "Fitness",
            "Food Service",
            "Forestry",
            "Gaming",
            "Government",
            "Graphic Design",
            "Healthcare",
            "Hospitality",
            "Human Resources",
            "Industrial Design",
            "Information Technology",
            "Insurance",
            "Interior Design",
            "International Relations",
            "Journalism",
            "Law Enforcement",
            "Linguistics",
            "Logistics",
            "Manufacturing",
            "Marine Biology",
            "Marketing",
            "Materials Science",
            "Mathematics",
            "Mechanical Engineering",
            "Media",
            "Medicine",
            "Mental Health",
            "Mining",
            "Music",
            "Nanotechnology",
            "Natural Language Processing",
            "Neuroscience",
            "Non-profit",
            "Nuclear Engineering",
            "Nutrition",
            "Oil & Gas",
            "Pharmaceuticals",
            "Philosophy",
            "Photography",
            "Physics",
            "Politics",
            "Psychology",
            "Public Health",
            "Public Relations",
            "Publishing",
            "Quantum Computing",
            "Real Estate",
            "Renewable Energy",
            "Retail",
            "Robotics",
            "Sales",
            "Science Communication",
            "Social Media",
            "Social Work",
            "Software Development",
            "Space Exploration",
            "Sports",
            "Supply Chain",
            "Telecommunications",
            "Textiles",
            "Tourism",
            "Transportation",
            "Urban Planning",
            "UX/UI Design",
            "Veterinary Medicine",
            "Video Production",
            "Virtual Reality",
            "Web Development",
            "Wildlife Conservation",
            "Acoustics",
            "Aeronautics",
            "AgriTech",
            "Animal Husbandry",
            "Anthropology",
            "Archaeology",
            "Astrophysics",
            "Augmented Reality",
            "Aviation",
            "Bioinformatics",
            "Biomedical Engineering",
            "Botany",
            "Business Intelligence",
            "Cartography",
            "Chemical Engineering",
            "Computer Vision",
            "Criminology",
            "Cryptography",
            "Culinary Arts",
            "Customer Relationship Management (CRM)",
            "Data Science",
            "Dentistry",
            "Digital Forensics",
            "E-learning",
            "Ecology",
            "Education Technology",
            "Emergency Services",
            "Energy Storage",
            "Epidemiology",
            "Ergonomics",
            "Ethics",
            "Facility Management",
            "Finance Technology (FinTech)",
            "Fisheries",
            "Food Technology",
            "Game Development",
            "Genomics",
            "Geology",
            "Geopolitics",
            "Gerontology",
            "Green Technology",
            "Horticulture",
            "Hydrology",
            "Industrial Automation",
            "Inventory Management",
            "IoT (Internet of Things)",
            "Landscape Architecture",
            "Library Science",
            "Machine Learning",
            "Meteorology",
            "Microbiology",
            "Mobile Development",
            "Oceanography",
            "Operations Research",
            "Optics",
            "Paleontology",
            "Performing Arts",
            "Petroleum Engineering",
            "Pharmacology",
            "Political Science",
            "Project Management",
            "Quality Assurance",
            "Recycling",
            "Remote Sensing",
            "Risk Management",
            "Security Systems",
            "Sociology",
            "Speech Recognition",
            "Sports Analytics",
            "Sustainable Development",
            "Taxation",
            "Thermodynamics",
            "Travel Technology",
            "Veterinary Technology",
            "Waste Management",
            "Water Resources",
            "Zoology",
        ];

        Self {
            domains: default_domains.into_iter().map(String::from).collect(),
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path)
            .with_context(|| format!("Failed to open config file: {}", path.as_ref().display()))?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(&path).with_context(|| {
            format!("Failed to create config file: {}", path.as_ref().display())
        })?;
        serde_json::to_writer_pretty(file, self)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;
        Ok(())
    }

    /// Get the default configuration file path
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("projector")
            .join("config.json")
    }
}
