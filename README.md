# Rust Shell

A simple, lightweight command-line shell written in Rust.

This project implements a custom shell with basic parsing, built-in commands, and an interactive REPL using [`rustyline`](https://github.com/kkawakam/rustyline) for history, completion, hints, and input validation.

## Features

- **Interactive REPL**: Reads commands from the user and executes them.
- **Built-in Commands**:
  - `cd` - Change the current directory.
  - `pwd` - Print the working directory.
  - `echo` - Print text to standard output.
  - `type` - Shows whether a command is a builtin or external executable.
  - `exit` - Terminate the shell.
- **External Command Execution**: Spawns and runs external binaries from the system `$PATH`.
- **Pipeline Support**: Supports simple pipelines using `|` between commands.
- **Persistent History**: Saves command history across sessions in `.shell_history`.
- **Rustyline Integration**: Provides completion, history hints, syntax highlighting, and basic input validation.

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

The codebase is organized into modules with clear responsibility boundaries:

- `src/main.rs` & `src/lib.rs`: Entry points and module declarations.
- `src/cli/`: REPL loop and Rustyline helper components for completion, hints, validation, and highlighting.
- `src/builtins/`: Built-in command implementations.
- `src/lexer/`: Tokenizes user input.
- `src/parser/`: Builds an AST from tokens and supports pipelines and command separators.
- `src/exec/`: Executes built-ins and external commands and handles pipelines.
- `src/error/`: Defines shell error types.
- `src/utils/`: Shared utility functions.

## Dependencies

- [`anyhow`](https://crates.io/crates/anyhow): Flexible error handling.
- [`rustyline`](https://crates.io/crates/rustyline): Readline implementation for Rust.
- [`thiserror`](https://crates.io/crates/thiserror): Derive macros for custom errors.
- [`bytes`](https://crates.io/crates/bytes): Utilities for working with bytes.
- [`pathsearch`](https://crates.io/crates/pathsearch): Finds executables in the system PATH.

