# LLM-Powered Dynamic Project Definition Wizard

An intelligent, interactive wizard driven by a Large Language Model (LLM) that guides users through a context-sensitive sequence of questions, ultimately generating a comprehensive and customized project definition for an LLM-based application or system.

## Features

### Core Workflow
- Dynamic question generation based on:
  - User-provided starting hints (optional)
  - Accumulated context from previous answers
  - Domain-specific knowledge (if a domain is selected)
- Various question types:
  - Multiple-choice
  - Yes/No
  - Rating scales
  - Free text entries

### Adaptive Question Flow
- Context-aware questions: each answer influences the next question
- Configurable number of questions
- "Back" feature to revise earlier answers

### LLM-Generated Project Output
The wizard generates a detailed project definition document that includes:
- Project name and short summary
- Use cases and goals (with examples or scenarios)
- Target user profile(s)
- Required inputs and expected outputs
- Functional components/modules
- Prompt engineering strategy
- Dataset needs and sources
- Evaluation metrics and success criteria
- Scalability and deployment recommendations
- Ethical and bias considerations

### Additional Features
- Predefined Templates/Presets: Start with industry-specific wizards
- Persona Modes: Different questioning styles (Product Manager, LLM Architect, UX Designer, Compliance Officer)
- Session Export Options: Save as Markdown
- Confidence Scoring: Each section includes a confidence level based on the specificity of user answers

## Installation

1. Make sure you have Rust installed. If not, install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Clone this repository:
   ```
   git clone <repository-url>
   cd Projector
   ```

3. Create a `.env` file in the project root with your OpenAI API key:
   ```
   OPENAI_API_KEY=your_api_key_here
   ```

4. Build the project:
   ```
   cargo build --release
   ```

## Usage

### Starting a New Session

```
cargo run -- new [OPTIONS]
```

Options:
- `-h, --hints <HINTS>`: Starting hints for the wizard
- `-d, --domain <DOMAIN>`: Domain for the project
- `-q, --questions <QUESTIONS>`: Maximum number of questions (default: 10)
- `-t, --template <TEMPLATE>`: Use a template
- `-p, --persona <PERSONA>`: Persona mode (product_manager, llm_architect, ux_designer, compliance_officer)
- `-o, --output <OUTPUT>`: Output file for the project definition

### Continuing an Existing Session

```
cargo run -- continue -s <SESSION_FILE> [-o <OUTPUT_FILE>]
```

### Listing Available Templates

```
cargo run -- templates
```

## Example

```
cargo run -- new --hints "I want to build a chatbot that helps users learn a new language" --questions 15 --persona ux_designer
```

## License

MIT