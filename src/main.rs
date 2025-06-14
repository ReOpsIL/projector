use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use dotenv::dotenv;
use std::path::PathBuf;
use tokio::runtime::Runtime;

mod wizard;

use wizard::context::{Context, Persona};
use wizard::llm::{LlmClient, LlmConfig};
use wizard::question::QuestionType;
use wizard::session::{Session, SessionManager};
use wizard::template::{Template, TemplateRepository};

/// LLM-Powered Dynamic Project Definition Wizard
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new wizard session
    New {
        /// Starting hints for the wizard
        #[clap(short = 'i', long)]
        hints: Option<String>,

        /// Domain for the project
        #[clap(short, long)]
        domain: Option<String>,

        /// Maximum number of questions
        #[clap(short, long, default_value = "10")]
        questions: usize,

        /// Use a template
        #[clap(short, long)]
        template: Option<String>,

        /// Persona mode
        #[clap(short, long)]
        persona: Option<String>,

        /// Output file for the project definition
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    /// Continue an existing wizard session
    Continue {
        /// Path to the session file
        #[clap(short, long)]
        session: PathBuf,

        /// Output file for the project definition
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    /// List available templates
    Templates,
}

fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse();

    // Create tokio runtime
    let runtime = Runtime::new().context("Failed to create Tokio runtime")?;

    // Execute the command
    match cli.command {
        Commands::New {
            hints,
            domain,
            questions,
            template,
            persona,
            output,
        } => runtime.block_on(new_session(hints, domain, questions, template, persona, output)),
        Commands::Continue { session, output } => {
            runtime.block_on(continue_session(session, output))
        }
        Commands::Templates => list_templates(),
    }
}

/// Start a new wizard session
async fn new_session(
    hints: Option<String>,
    domain: Option<String>,
    max_questions: usize,
    template_name: Option<String>,
    persona_name: Option<String>,
    output_path: Option<PathBuf>,
) -> Result<()> {
    println!("🧙 Starting LLM-Powered Project Definition Wizard");

    // Create LLM client
    let llm_client = create_llm_client()?;

    // Create session
    let mut session = if let Some(template_name) = template_name {
        // Create session from template
        let repo = TemplateRepository::new();
        let template = repo
            .get_template(&template_name)
            .context(format!("Template '{}' not found", template_name))?;

        println!("Using template: {}", template.name);
        println!("Description: {}", template.description);

        Session::from_template(template)
    } else {
        // Create new session
        let mut context = Context::new();

        // Set starting hints if provided
        if let Some(hints) = hints {
            context = Context::with_hints(hints);
        }

        // Set domain if provided
        if let Some(domain) = domain {
            context = Context::with_domain(domain);
        }

        Session::with_context(context)
    }
    .with_max_questions(max_questions);

    // Set persona if provided
    if let Some(persona_name) = persona_name {
        let persona = match persona_name.to_lowercase().as_str() {
            "pm" | "product" | "product_manager" => Persona::ProductManager,
            "architect" | "llm_architect" => Persona::LlmArchitect,
            "ux" | "designer" | "ux_designer" => Persona::UxDesigner,
            "compliance" | "compliance_officer" => Persona::ComplianceOfficer,
            _ => Persona::Default,
        };

        println!(
            "Using persona: {}",
            match persona {
                Persona::Default => "Default",
                Persona::ProductManager => "Product Manager",
                Persona::LlmArchitect => "LLM Architect",
                Persona::UxDesigner => "UX Designer",
                Persona::ComplianceOfficer => "Compliance Officer",
            }
        );

        session.context.persona = persona;
    }

    // Run the wizard
    run_wizard(session, llm_client, output_path).await
}

/// Continue an existing wizard session
async fn continue_session(session_path: PathBuf, output_path: Option<PathBuf>) -> Result<()> {
    println!("🧙 Continuing LLM-Powered Project Definition Wizard");

    // Load session
    let session = Session::load_from_file(session_path)
        .context("Failed to load session file")?;

    // Create LLM client
    let llm_client = create_llm_client()?;

    // Run the wizard
    run_wizard(session, llm_client, output_path).await
}

/// List available templates
fn list_templates() -> Result<()> {
    println!("🧙 Available Templates");

    let repo = TemplateRepository::new();
    let templates = repo.get_all_templates();

    if templates.is_empty() {
        println!("No templates available");
    } else {
        for (i, template) in templates.iter().enumerate() {
            println!("{}. {} ({})", i + 1, template.name, template.domain);
            println!("   {}", template.description);
            println!();
        }
    }

    Ok(())
}

/// Create an LLM client
fn create_llm_client() -> Result<LlmClient> {
    // Get API key from environment
    let api_key = std::env::var("OPENAI_API_KEY").ok();

    // Create config
    let config = LlmConfig {
        api_key,
        ..LlmConfig::default()
    };

    // Create client
    LlmClient::with_config(config)
}

/// Run the wizard
async fn run_wizard(
    session: Session,
    llm_client: LlmClient,
    output_path: Option<PathBuf>,
) -> Result<()> {
    let mut session_manager = SessionManager::new(session, llm_client);
    session_manager.start();

    let theme = ColorfulTheme::default();

    println!("Starting wizard session with {} questions", session_manager.max_questions());
    println!("Type 'back' to go back to a previous question");
    println!("Type 'quit' to exit the wizard");
    println!();

    // Question loop
    loop {
        // Check if we've reached the maximum number of questions
        let current_count = session_manager.question_count();
        let max_questions = session_manager.max_questions();

        if current_count >= max_questions {
            println!("Maximum number of questions reached");
            break;
        }

        // Generate next question
        let question = match session_manager.generate_next_question().await {
            Ok(q) => q,
            Err(e) => {
                println!("Error generating question: {}", e);
                break;
            }
        };

        // Display question
        println!("Question {}/{}: {}", current_count + 1, max_questions, question.text);

        if let Some(help_text) = &question.help_text {
            println!("Hint: {}", help_text);
        }

        // Get user response based on question type
        let response = match question.question_type {
            QuestionType::MultipleChoice => {
                if let Some(options) = &question.options {
                    let selection = Select::with_theme(&theme)
                        .items(options)
                        .default(0)
                        .interact()
                        .context("Failed to get user input")?;
                    options[selection].clone()
                } else {
                    "Invalid question: missing options".to_string()
                }
            }
            QuestionType::YesNo => {
                let confirmed = Confirm::with_theme(&theme)
                    .with_prompt("Yes or No?")
                    .default(true)
                    .interact()
                    .context("Failed to get user input")?;
                if confirmed {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                }
            }
            QuestionType::RatingScale => {
                if let Some((min, max)) = question.scale {
                    let options: Vec<String> = (min..=max)
                        .map(|n| format!("{}", n))
                        .collect();
                    let selection = Select::with_theme(&theme)
                        .items(&options)
                        .default(0)
                        .interact()
                        .context("Failed to get user input")?;
                    options[selection].clone()
                } else {
                    "Invalid question: missing scale".to_string()
                }
            }
            QuestionType::FreeText => {
                let input: String = Input::with_theme(&theme)
                    .with_prompt("Your answer")
                    .interact_text()
                    .context("Failed to get user input")?;

                // Check for special commands
                if input.trim().to_lowercase() == "back" {
                    // Go back to previous question
                    match session_manager.go_back() {
                        Ok(_) => {
                            println!("Going back to previous question");
                            continue;
                        }
                        Err(e) => {
                            println!("Cannot go back: {}", e);
                            continue;
                        }
                    }
                } else if input.trim().to_lowercase() == "quit" {
                    // Exit the wizard
                    println!("Exiting wizard");
                    return Ok(());
                }

                input
            }
        };

        // Answer the question
        if let Err(e) = session_manager.answer_question(response) {
            println!("Error answering question: {}", e);
            break;
        }

        println!();
    }

    // Generate project definition
    println!("Generating project definition...");
    let markdown = match session_manager.generate_project_definition().await {
        Ok(md) => md,
        Err(e) => {
            println!("Error generating project definition: {}", e);
            return Err(e);
        }
    };

    // Display project definition
    println!("\n{}\n", markdown);

    // Save to file if output path is provided
    if let Some(path) = output_path {
        println!("Saving project definition to {}", path.display());
        session_manager.export_output(path)?;
    }

    // Ask if user wants to save the session
    let save_session = Confirm::with_theme(&theme)
        .with_prompt("Do you want to save this session for later?")
        .default(false)
        .interact()
        .context("Failed to get user input")?;

    if save_session {
        let session_path: String = Input::with_theme(&theme)
            .with_prompt("Enter path to save session")
            .default("wizard_session.json".to_string())
            .interact_text()
            .context("Failed to get user input")?;

        println!("Saving session to {}", session_path);
        session_manager.session.save_to_file(session_path)?;
    }

    println!("Wizard completed successfully!");
    Ok(())
}
