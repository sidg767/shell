# Rust Shell

A simple, lightweight, and extensible command-line shell written in Rust.

This project implements a custom shell with basic pipeline support, built-in commands, and an interactive REPL using [`rustyline`](https://github.com/kkawakam/rustyline) to provide advanced terminal features such as history, auto-completion, hinting, syntax highlighting, and input validation.

## Features

- **Interactive REPL**: A responsive command-line interface with a dynamic prompt displaying the current working directory.
- **Built-in Commands**: Fast, native execution of common utilities:
  - `cd` - Change the current directory.
  - `pwd` - Print the working directory.
  - `echo` - Print text to standard output.
  - `type` - Indicate how a command name is interpreted (builtin or external executable).
  - `exit` - Terminate the shell.
- **External Command Execution**: Seamlessly spawns and executes external binaries found in your system's `$PATH`.
- **Pipeline Support**: Supports basic piping (`|`) between commands, chaining the standard output of the left command to the standard input of the right command.
- **Persistent History**: Keeps track of your command history across sessions (stored in `.shell_history`).
- **Extensible Architecture**: Modular design separating lexical analysis (`lexer`), parsing (`parser`), environment management (`env`), and command execution (`exec`).

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (cargo, rustc) installed on your system.

### Build and Run

1. **Clone the repository** (if you haven't already):
   ```bash
   git clone https://github.com/sidg767/shell
   cd shell
   ```

2. **Build the project** using Cargo:
   ```bash
   cargo build --release
   ```

3. **Run the shell**:
   ```bash
   cargo run
   ```
   Or run the compiled binary directly:
   ```bash
   ./target/release/shell
   ```

## Project Structure

The codebase is organized into several modules for clear separation of concerns:

- `src/main.rs` & `src/lib.rs`: Entry points and module declarations.
- `src/cli/`: Handles the interactive REPL loop, terminal input, and Rustyline integration.
- `src/builtins/`: Implementation of built-in commands (`cd`, `echo`, `pwd`, `type`).
- `src/lexer/`: Tokenizer responsible for lexical analysis of user input.
- `src/parser/`: Converts tokens into an Abstract Syntax Tree (AST).
- `src/exec/`: Handles the execution of commands, processes, and pipelines.
- `src/env/`: Environment variable management.
- `src/jobs/`: Job control (background/foreground processes).
- `src/signals/`: Terminal signal handling (e.g., `SIGINT`, `SIGTERM`).
- `src/error/`: Centralized error handling types and utilities.
- `src/utils/`: Common helper functions used across the project.

## Dependencies

- [`anyhow`](https://crates.io/crates/anyhow): Flexible error handling.
- [`rustyline`](https://crates.io/crates/rustyline): Readline implementation for Rust.
- [`thiserror`](https://crates.io/crates/thiserror): Derive macros for custom errors.
- [`bytes`](https://crates.io/crates/bytes): Utilities for working with bytes.
- [`pathsearch`](https://crates.io/crates/pathsearch): Simple tool to find executables in the system PATH.

