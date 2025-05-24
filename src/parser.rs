use std::fmt;

use crate::{lexer::{Lexer, Token}, operator::Op};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Atom(char),
    Operation(Op, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?;
                }
                write!(f, ")")
            }
        }
    }
}


pub fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        Token::Op(Op::LBra) => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Op(Op::RBra));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(Op::RBra) => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        let (l_bp, r_bp) = op.infix_binding_power();
        if l_bp < min_bp {
            break;
        }
        lexer.next();

        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }

    lhs
}
