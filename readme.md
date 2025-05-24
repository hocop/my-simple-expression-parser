# Simple Expression Evaluator

An exercise, following this video: [This Simple Algorithm Powers Real Interpreters: Pratt Parsing](https://www.youtube.com/watch?v=0c8b7YfsBKs)

## Features
- Evaluates basic arithmetic expressions.
- Supports addition, subtraction, multiplication, and division.
- Handles operator precedence and parentheses.

## Limitations
- Only integers from 0 to 9 are supported.

## Example
```bash
$ cargo run
>>> 5 + 9 * 5
50
>>> (1 + 2) * 3
9
```