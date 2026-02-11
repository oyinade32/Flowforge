# Flowforge  
### A Rust-Based Command-Line Tool for Project Exploration and AI-Assisted Understanding

---

## Overview

Flowforge is a Rust-based command-line interface (CLI) designed to help developers and students quickly understand new or unfamiliar software projects.

When joining an existing project, developers often struggle to identify:
- Project structure
- File and module responsibilities
- Entry points and configuration files
- Relationships between components

Flowforge addresses this challenge by combining structured CLI commands with an experimental AI-style explanation engine that guides users through a project’s architecture.

---

## Problem Statement

Understanding a new codebase is time-consuming and error-prone, especially for:
- Students learning software engineering
- Developers onboarding into new teams
- Open-source contributors reviewing repositories

Manual exploration of files and folders slows productivity and creates onboarding friction.

---

## Solution

Flowforge provides a command-line workflow that allows users to:

- Initialize and scaffold projects with consistent structure
- Explore files and directories via simple CLI commands
- Invoke an AI-style agent to explain project layout and intent
- Route user input through a modular reasoning pipeline

The tool is written entirely in Rust, ensuring performance, safety, and cross-platform compatibility.

---

## Features

- Command-line driven project interaction
- Modular Rust architecture
- AI-style explanation and reasoning engine (mock implementation)
- Intent detection (explain, summarize, analyze, ask)
- Extensible design for future AI integration
- Git-friendly development workflow

---

## AI Assistant (Experimental)

Flowforge includes an experimental AI-style assistant designed to simulate intelligent project understanding.

The current implementation:
- Parses user input from the CLI
- Detects user intent (explain, summarize, analyze, ask)
- Routes requests through a structured AI engine
- Generates contextual responses based on detected intent

Although the AI engine is currently a mock implementation, its architecture is intentionally designed to support future integration with:
- Local language models
- External AI APIs
- System-aware developer tooling

This ensures Flowforge remains flexible and provider-agnostic.

---

## Design Intent

Flowforge is intentionally designed as a lightweight, local-first CLI tool.

Unlike cloud-dependent AI coding assistants, Flowforge:
- Does not require an internet connection
- Does not depend on a specific AI provider
- Focuses on understanding *existing* project structure rather than generating new code

The current AI engine is a mock implementation that demonstrates:
- Intent detection
- Command routing
- Modular reasoning stages

This architecture allows future integration with:
- Local language models
- Rule-based analyzers
- External APIs (optional, not required)

The goal is not to replace tools like Gemini CLI or Claude Code, but to complement them by providing structured, system-aware project understanding directly in the terminal.

---

# CLI Command Reference

The following commands are available in Flowforge:

| Command | Description |
|--------|-------------|
|`flowforge ai help` | Displays available AI-related commands |
| `flowforge ai ask <question>` | Sends a question to the AI-style engine for processing |
| `flowforge ai explain`| Reads and explains the project README file |
| `flowforge help`| Displays general Flowforge CLI help information |

These commands are designed to be simple and discoverable, allowing users to explore projects incrementally through guided CLI interactions.

## Limitations & Future Work

Flowforge is intentionally designed as a lightweight, local-first CLI tool.

### Current Limitations
- The AI assistant is a mock implementation and does not use a real language model
- Explanations are based on structured intent detection rather than semantic understanding
- Project analysis is limited to user-invoked commands (no background indexing)
- The tool does not yet analyze source code dependencies or execution flow

### Future Enhancements
- Integration with local or API-based language models
- Static analysis of project files and module relationships
- Context-aware explanations based on project history and user commands
- Expanded CLI commands for deeper project inspection
- Optional configuration via a `flowforge.toml` file

## Installation

Clone the repository and build the project locally:

```bash
git clone https://github.com/oyinade32/flowforge.git
cd flowforge
cargo build


## Download Prebuilt Binary

You can download the precompiled Linux binary from the latest GitHub release:

https://github.com/oyinade32/Flowforge/releases/latest


---

## License

This project was developed for academic purposes as part of a university examination.
