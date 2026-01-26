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

## Installation

Clone the repository and build the project locally:

```bash
git clone https://github.com/oyinade32/flowforge.git
cd flowforge
cargo build

---

## License

This project was developed for academic purposes as part of a university examination.
