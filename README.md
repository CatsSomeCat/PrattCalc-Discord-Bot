# PrattCalc Discord Bot

A Discord bot calculator using Pratt Parsing for expression evaluation, fully implemented in Rust.

## Overview

PrattCalc is a powerful calculator Discord bot with a full expression language featuring variables, control flow, and mathematical functions.

The bot utilizes Pratt Parsing (top-down operator precedence parsing) for efficient and extensible expression evaluation.

### Features

- **Arithmetic Operations**: Basic `+`, `-`, `*`, `/`, `%`, `^` operations
- **Variables & Constants**: Declare and use variables, with built-in mathematical constants (`PI`, `E`, etc.)
- **Control Flow**: `if`/`else`, `while` loops, and `break`/`continue` statements
- **Mathematical Functions**: `sin`, `cos`, `tan`, `log`, `sqrt`, and many more
- **Special Operations**: Support for both prefix and infix operators
- **Comment Support**: Both line (`//`) and block (`/* */`) comments

Additionally, you can right-click on any message containing code and select "Apps > Execute Code" to run the code in the message.

## Technical Implementation

- Core calculator engine built in Rust
- Pratt Parsing algorithm for expression evaluation
- Discord bot interface using [Serenity](https://github.com/serenity-rs/serenity) v0.12
- Async runtime with Tokio
- Comprehensive test suite ensuring reliability

## Project Structure

The codebase is organized into the following main components:

- `src/core/` - The core calculator implementation
  - Expression parser and evaluator
  - Symbol table for variable management
  - AST (Abstract Syntax Tree) representation
  - Error handling

- `src/discord/` - Discord bot integration
  - Command handling
  - User session management
  - Help system
  - Bot event handlers

- `src/bin/` - Binary executables
  - CLI calculator interface

- `tests/` - Comprehensive test suite

## Getting Started

### Prerequisites

- Rust 1.65 or newer
- Discord Bot Token (for bot functionality)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/CatsSomeCat/PrattCalc-Discord-Bot.git
   cd PrattCalc-Discord-Bot
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Configure the Discord bot:
   - Create a `.env` file in the project root
   - Add your Discord token: `DISCORD_TOKEN=your_token_here`

4. Run the bot:
   ```bash
   cargo run --release --bin ppaaeedb
   ```

### CLI Usage

You can also use PrattCalc as a standalone CLI calculator:

```bash
cargo run --release --bin ppaaeecli --features cli
```

## Test Organization

The tests for the PrattCalc Discord bot are organized into the following main files:

### 1. Language Features Tests (`language_features_tests.rs`)

Tests for the core language features including:
- Expression evaluation and operators
- Variables and assignments
- Constants and their behavior
- Basic arithmetic operations
- Comparison operators

### 2. Control Flow Tests (`control_flow_tests.rs`)

Tests for various control flow constructs:
- If-Else statements
- While loops
- Break/Continue statements
- Complex nested control structures

### 3. Advanced Features Tests (`advanced_features_tests.rs`)

Tests for more complex language features:
- Mathematical functions (sine, cosine, etc.)
- Complex expressions with multiple operators
- Variable scoping
- Block-level shadowing
- Nested operations

### 4. Comment Support Tests (`comment_tests.rs`)

Tests for the comment functionality:
- Line comments (`//`)
- Block comments (`/* */`)
- Comments with mathematical symbols
- Comments at the end of expressions

### 5. Logical Operations Tests (`logical_operations_tests.rs`)

Tests for logical operations:
- Boolean operators (`&&`, `||`, `!`)
- Comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- Compound logical expressions

## Running Tests

You can run all tests with:
```bash
cargo test
```

Or run a specific test file with:
```bash
cargo test --test control_flow_tests
```

Or run a specific test with:
```bash
cargo test --test control_flow_tests test_break_in_if_inside_while
```

## License

This project is licensed under the MIT License; see the LICENSE file for details. 
