# rulc
### Easy to use TUI REPL calculator with plot support

![pictures/img.png](pictures/img.png)

## Installation
cargo: `cargo install rulc`

## Usage
- `rulc` for REPL mode
- `rulc --tui` for TUI mode
- `rulc --exec <expression>` for inline mode

## Operators

| Operator       | Syntax  | Example          |
| -------------- | ------- | ---------------- |
| Addition       | `a + b` | `2 + 3` → `5`   |
| Subtraction    | `a - b` | `10 - 4` → `6`  |
| Multiplication | `a * b` | `3 * 4` → `12`  |
| Division       | `a / b` | `9 / 2` → `4.5` |
| Exponentiation | `a ^ b` | `2 ^ 8` → `256` |
| Unary minus    | `-a`    | `-5` → `-5`     |

## Built-in functions

| Function       | Syntax   | Description           |
| -------------- | -------- | --------------------- |
| Sine           | `sin(x)` | Sine of x (radians)   |
| Cosine         | `cos(x)` | Cosine of x (radians) |
| Tangent        | `tan(x)` | Tangent of x (radians)|
| Arcsine        | `asin(x)`| Inverse sine          |
| Arccosine      | `acos(x)`| Inverse cosine        |
| Arctangent     | `atan(x)`| Inverse tangent       |
| Square root    | `sqrt(x)`| Square root of x      |
| Natural log    | `ln(x)`  | Logarithm base e      |
| Log base 10    | `log(x)` | Logarithm base 10     |
| Absolute value | `abs(x)` | \|x\|                 |
| Ceiling        | `ceil(x)`| Round up to integer   |
| Floor          | `floor(x)`| Round down to integer|

## Built-in constants

| Constant       | Syntax | Value     |
| -------------- | ------ | --------- |
| Pi             | `pi`   | 3.14159…  |
| Euler's number | `e`    | 2.71828…  |

## Variables and functions

Assign a variable:
```
x = 5
y = x * 2 + 1
```

Compound assignment:
```
x += 10
x *= 2
```

Define a custom function:
```
f(x) = x^2 + 2*x + 1
g(x) = sin(x) / x
```

## Plotting graphs

Plots are available in TUI mode (`rulc --tui`).

**Syntax:**
```
draw <function> from <expr> to <expr>
```

**Examples:**

Plot a built-in function:
```
draw sin from -pi to pi
```

Define and plot a custom function:
```
f(x) = x^2 - 4
draw f from -5 to 5
```

Plot using a computed range:
```
draw cos from -2*pi to 2*pi
```

After entering a `draw` command the chart panel updates automatically. The x and y axes scale to fit the plotted range, and the zero axes are shown in gray when they fall within the visible area.

## Project structure 
```
├── core
└── ├── evaluator           // evaluates parsed expressions
└── ├── lexer               // tokenizes input
└── ├── operations          // defines arithmetic operations
└── ├── parser              // parses tokenized input
└── ├── ├── numeric         // parses numeric expressions
└── ├── registries          // registries for identifiers and operation 
└── view                    // program modes (inline, REPL, TUI) 
└── main.rs                 // program entry point
```

## Evaluation algorithm
1. Tokenize the input using the lexer
2. Parse the tokenized input using the parser
3. Evaluate the parsed expression using the evaluator (Pratt expression parser)
