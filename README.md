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
- **Functions & Procedures**: Define reusable code with `fn` and `proc` keywords

### Discord Slash Commands

PrattCalc implements the following slash commands:

- `/evaluate <expression>` - Calculate the result of a mathematical expression
- `/execute <code>` - Run multiline code blocks with complex logic
- `/help <topic>` - Get help on a specific topic or general usage information
- `/vars` - List all currently defined variables in your session
- `/clear` - Clear all variables and history in your current session
- `/statistics` - Display bot statistics and system information

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
   - Add your Discord token: `DISCORD_TOKEN = your_token_here`

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

The tests for the PrattCalc Discord bot are organized into two main files:

### 1. Basic Features Tests (`basic_features_tests.rs`)

Tests for the core language features including:
- Expression evaluation and operators
- Variables and assignments
- Constants and their behavior
- Basic arithmetic operations
- Comparison operators
- Control flow structures (if/else, while loops)
- Error handling

### 2. Advanced Features Tests (`advanced_features_tests.rs`)

Tests for more complex language features:
- Mathematical functions (sine, cosine, etc.)
- Complex expressions with multiple operators
- Variable scoping and block-level shadowing
- Functions and procedures
- End keyword for flow control
- Nested scopes and variable visibility
- Constant shadowing and immutability

## Running Tests

You can run all tests with:
```bash
cargo test
```

Or run a specific test file with:
```bash
cargo test --test advanced_features_tests
```

Or run a specific test with:
```bash
cargo test --test advanced_features_tests test_random_number_generator
```

## Language Examples

Here are more comprehensive examples of the language features available in PrattCalc.

### Variables and Constants

```rust
// Variable declaration and assignment
let x = 10;
let y = 20;
x = x + 5;  // x is now 15

// Constants (immutable)
const PI_SQUARED = 9.8696;
const GRAVITY = 9.81;

// Built-in mathematical constants
let circle_area = PI * radius * radius;
let exponential = E * 2;
let golden_ratio = PHI;
```

### Mathematical Functions

```rust
// Trigonometric functions
let sine = sin(PI / 2);  // 1.0
let cosine = cos(0);     // 1.0
let tangent = tan(PI / 4); // 1.0

// Logarithms and powers
let natural_log = log(E);     // 1.0
let square_root = sqrt(16);   // 4.0
let power = 2 ^ 8;            // 256

// Min, max, and absolute value
let minimum = min(10, -5);    // -5
let maximum = max(10, -5);    // 10
let absolute = abs(-25);      // 25

// Random numbers
let random_value = rand();       // 0.0 to 1.0
let random_range = rand(10, 20); // 10.0 to 20.0
```

### Control Flow

```rust
// If-else statements
let x = 10;
let result = 0;

if x > 5 {
    result = 1;
} else {
    result = 0;
}

// Nested if statements
if x > 0 {
    if x < 10 {
        result = 1;
    } else {
        result = 2;
    }
} else {
    result = 0;
}

// While loops
let sum = 0;
let i = 1;

while i <= 10 {
    sum = sum + i;
    i = i + 1;
}

// sum is now 55
end sum;
```

### Block Scopes and Shadowing

```rust
let x = 5;
{
    let x = 10;  // Shadows outer x
    let y = 20;
    // x is 10 here
}
// x is 5 here, y is not accessible

// Accessing outer variables
let a = 100;
{
    let b = a + 50;  // b is 150, using outer a
    a = a * 2;       // Updates outer a to 200
}
```

### Using the End Keyword

```rust
// End keyword terminates program execution and returns the value
let x = 10;
let y = 20;

// This will terminate the program and return 30
end x + y;

// Code here never executes
let z = 30;
```

### Function and Procedure Examples

#### Functions (return values)

```rust
// Define a function that calculates the area of a rectangle
fn area(width, height) {
    return width * height
}

// Define a function with conditional returns
fn abs(x) {
    if x < 0 {
        return -x;
    }

    return x
}

// Recursive factorial function
fn factorial(n) {
    if n <= 1 {
        return 1;
    }

    return n * factorial(n - 1)
}

// Function that uses other functions
fn hypotenuse(a, b) {
    return sqrt(a * a + b * b)
}

// Use the function
let rectangle_area = area(5, 10);  // Returns 50
```

#### Procedures (no return values)

```rust
// Define a procedure to initialize values
proc init_values(a, b) {
    let sum = a + b; let product = a * b;
}

// Define a procedure that modifies outer variables
let total = 0;
proc add_to_total(value) {
    total = total + value;
}

// Use the procedure
add_to_total(5); add_to_total(10);  // total is now 15
```

#### Functions and Procedures Working Together

```rust
// Global state shared between functions and procedures
let sum = 0;
let count = 0;

// A function that performs a calculation
fn square(x) {
    return x * x
}

// A procedure that uses function results
proc process_number(x) {
    let squared = square(x);
    sum = sum + squared;
    count = count + 1;
}

// A function that uses state modified by procedures
fn get_average() {
    if count == 0 {
        return 0;
    }

    return sum / count
}

// Use them together
process_number(3);  // sum = 9, count = 1
process_number(4);  // sum = 9 + 16 = 25, count = 2
process_number(5);  // sum = 25 + 25 = 50, count = 3

// Get the average
let avg = get_average();  // 50 / 3 ≈ 16.67
```

### Complex Expressions

```rust
// Mixed operations with proper precedence
let result = 2 + 3 * 4 ^ 2 - 8 / 2;  // 2 + 3 * 16 - 4 = 2 + 48 - 4 = 46

// Compound expressions with functions
let complex = sin(PI/4) * sqrt(2) + abs(-5) / log(E * 2);

// Chained function calls
let nested = sqrt(abs(sin(PI) * -10));  // sqrt(abs(-0 * -10)) = sqrt(0) = 0
```

### Important Notes

- There's no indexing or array access since the language doesn't have containers
- The language only supports numeric values (no strings or containers)
- The `end` keyword terminates the entire program execution and returns a value
- The `return` keyword is used only within functions to return a value from that function

## License

This project is licensed under the MIT License; see the LICENSE file for details.
