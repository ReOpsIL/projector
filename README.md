# Projector: LLM-Powered Dynamic Project Definition Wizard

Projector is a command-line tool that helps you define LLM-based applications through an interactive question-and-answer session. It uses an LLM to generate thoughtful, context-aware questions and ultimately produces a comprehensive project definition document.

## Features

- Interactive Q&A session to define your project
- Multiple question types (multiple choice, yes/no, rating scale, free text)
- Persona-based questioning (Product Manager, LLM Architect, UX Designer, Compliance Officer)
- Template support for common project types
- Markdown output for project definitions
- Session saving and resuming

## Installation

```bash
cargo install projector
```

## Usage

### Starting a new session

```bash
projector new [OPTIONS]
```

Options:
- `-i, --hints <HINTS>`: Starting hints for the wizard
- `-d, --domain <DOMAIN>`: Domain for the project
- `-q, --questions <QUESTIONS>`: Maximum number of questions (default: 10)
- `-t, --template <TEMPLATE>`: Use a template
- `-p, --persona <PERSONA>`: Persona mode
- `-o, --output <OUTPUT>`: Output file for the project definition

### Continuing a session

```bash
projector continue --session <SESSION_FILE> [--output <OUTPUT>]
```

### Listing templates

```bash
projector templates
```

## Configuration

Projector uses OpenRouter as its LLM provider. You'll need to set up your API key in your environment:

```bash
export OPENROUTER_API_KEY=your_api_key_here
```

You can also create a `.env` file in the project directory with the following content:

```
OPENROUTER_API_KEY=your_api_key_here
```

## Models

By default, Projector uses the `google/gemini-2.5-flash-preview-05-20` model, but you can configure it to use any model supported by OpenRouter.

## License

[MIT License](LICENSE)