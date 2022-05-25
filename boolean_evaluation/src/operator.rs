use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Operator {
    Neg,
    And,
    Or,
    Xor,
    Imply,
    Leq,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Neg => write!(f, "¬"),
            Operator::And => write!(f, "∧"),
            Operator::Or => write!(f, "∨"),
            Operator::Xor => write!(f, "⊕"),
            Operator::Imply => write!(f, "→"),
            Operator::Leq => write!(f, "⇔"),
        }
    }
}
