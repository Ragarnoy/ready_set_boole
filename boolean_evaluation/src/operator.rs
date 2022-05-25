use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Not,
    And,
    Or,
    Xor,
    Imply,
    /// Bi-conditional
    Xnor,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Not => write!(f, "¬"),
            Operator::And => write!(f, "∧"),
            Operator::Or => write!(f, "∨"),
            Operator::Xor => write!(f, "⊕"),
            Operator::Imply => write!(f, "→"),
            Operator::Xnor => write!(f, "⇔"),
        }
    }
}
