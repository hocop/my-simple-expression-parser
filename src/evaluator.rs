use crate::{operator::Op, parser::Expression};


pub trait Eval {
    fn eval(&self) -> f32;
}

impl Eval for Expression {
    fn eval(&self) -> f32 {
        match self {
            Expression::Atom(c) => c.to_digit(10).unwrap() as f32,
            Expression::Operation(op, expressions) => {
                let lhs = expressions.first().unwrap();
                let rhs = expressions.last().unwrap();
                match op {
                    Op::Add => lhs.eval() + rhs.eval(),
                    Op::Sub => lhs.eval() - rhs.eval(),
                    Op::Mul => lhs.eval() * rhs.eval(),
                    Op::Div => lhs.eval() / rhs.eval(),
                    _ => panic!()
                }
            },
        }
    }
}
