use crate::operator::Op;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Atom(char),
    Op(Op),
    Eof,
}

pub struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(Op::from_char(c)),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}
