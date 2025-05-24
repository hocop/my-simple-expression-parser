use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    LBra,
    RBra,
}

impl Op {
    pub fn from_char(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            '(' => Op::LBra,
            ')' => Op::RBra,
            _ => panic!("not an operator: {}", c)
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
            Op::LBra => '(',
            Op::RBra => ')',
        }
    }

    pub fn infix_binding_power(self: Op) -> (f32, f32) {
        match self {
            Op::Add | Op::Sub => (1.0, 1.1),
            Op::Mul | Op::Div => (2.0, 2.1),
            _ => panic!("no binding for op: {:?}", self)
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
