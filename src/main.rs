use std::io::{self, Write};

use evaluator::Eval;
use lexer::Lexer;
use parser::parse_expression;

pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod operator;


fn main() {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let code = {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            buf
        };
        let mut lexer = Lexer::new(&code);
        let expr = parse_expression(&mut lexer, 0.0);
        println!("{}", expr.eval());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(code: &str) -> f32 {
        let mut lexer = Lexer::new(code);
        let expr = parse_expression(&mut lexer, 0.0);
        expr.eval()
    }

    #[test]
    fn test_addition() {
        assert_eq!(evaluate("1 + 2"), 3.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(evaluate("3 - 1"), 2.0);
        assert_eq!(evaluate("3 - 9"), -6.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(evaluate("2 * 3"), 6.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(evaluate("6 / 2"), 3.0);
        assert_eq!(evaluate("3 / 2"), 1.5);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate("(1 + 2) * 3"), 9.0);
        assert_eq!(evaluate("1 + (2 * 3)"), 7.0);
    }

    #[test]
    fn test_associativity_and_precedence() {
        assert_eq!(evaluate("1 + 2 * 3 + 4"), 11.0);
        assert_eq!(evaluate("1 * 2 + 3 * 4"), 14.0);
    }
}