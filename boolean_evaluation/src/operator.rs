use crate::set::Set;
use std::fmt;
use Operator::*;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Operator {
    Not,
    And,
    Or,
    Xor,
    Imply,
    Xnor,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Not => write!(f, "¬"),
            And => write!(f, "∧"),
            Or => write!(f, "∨"),
            Xor => write!(f, "⊕"),
            Imply => write!(f, "→"),
            Xnor => write!(f, "⇔"),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Not => write!(f, "!"),
            And => write!(f, "&"),
            Or => write!(f, "|"),
            Xor => write!(f, "^"),
            Imply => write!(f, ">"),
            Xnor => write!(f, "="),
        }
    }
}

impl Operator {
    pub fn eval_binary(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Imply => !lhs | rhs,
            Xnor => lhs == rhs,
            And => lhs & rhs,
            Xor => lhs ^ rhs,
            Or => lhs | rhs,
            _ => unreachable!(),
        }
    }

    pub fn eval_unary(self, child: bool) -> bool {
        match self {
            Not => !child,
            _ => unreachable!(),
        }
    }

    pub fn eval_binary_sets(self, lhs: Set, rhs: Set) -> Set {
        match self {
            And => lhs & rhs,
            Or => lhs | rhs,
            _ => unreachable!(),
        }
    }

    pub fn eval_unary_sets(self, mut child: Set) -> Set {
        match self {
            Not => {
                child.is_complement = !child.is_complement;
                child
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod operator_tests {
    use super::*;

    mod bool {
        use super::*;
        mod binary {
            use super::*;

            #[test]
            fn test_eval_and() {
                assert!(And.eval_binary(true, true));
                assert!(!And.eval_binary(true, false));
                assert!(!And.eval_binary(false, true));
                assert!(!And.eval_binary(false, false));
            }

            #[test]
            fn test_eval_or() {
                assert!(Or.eval_binary(true, true));
                assert!(Or.eval_binary(true, false));
                assert!(Or.eval_binary(false, true));
                assert!(!Or.eval_binary(false, false));
            }

            #[test]
            fn test_eval_xor() {
                assert!(!Xor.eval_binary(true, true));
                assert!(Xor.eval_binary(true, false));
                assert!(Xor.eval_binary(false, true));
                assert!(!Xor.eval_binary(false, false));
            }

            #[test]
            fn test_eval_imply() {
                assert!(Imply.eval_binary(true, true));
                assert!(!Imply.eval_binary(true, false));
                assert!(Imply.eval_binary(false, true));
                assert!(Imply.eval_binary(false, false));
            }

            #[test]
            fn test_eval_xnor() {
                assert!(Xnor.eval_binary(true, true));
                assert!(!Xnor.eval_binary(true, false));
                assert!(!Xnor.eval_binary(false, true));
                assert!(Xnor.eval_binary(false, false));
            }
        }
        mod unary {
            use super::*;

            #[test]
            fn test_eval_unary() {
                assert!(!Not.eval_unary(true));
                assert!(Not.eval_unary(false));
            }
        }
    }

    mod sets {
        use super::*;

        mod binary {
            use super::*;

            #[test]
            fn test_eval_and() {
                assert_eq!(
                    And
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![2, 3, 4])),
                    Set::from(vec![2, 3])
                );
                assert_eq!(
                    And
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![4, 5, 6])),
                    Set::from(vec![])
                );
                assert_eq!(
                    And.eval_binary_sets(
                        Set::from(vec![1, 2, 3]),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![4, 5, 6]).complement()
                );
                assert_eq!(
                    And.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6])
                    ),
                    Set::from(vec![1, 2, 3]).complement()
                );
                assert_eq!(
                    And.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![1, 2, 3, 4, 5, 6]).complement()
                );
            }

            #[test]
            fn test_eval_or() {
                assert_eq!(
                    Or
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![2, 3, 4])),
                    Set::from(vec![1, 2, 3, 4])
                );
                assert_eq!(
                    Or
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![4, 5, 6])),
                    Set::from(vec![1, 2, 3, 4, 5, 6])
                );
                assert_eq!(
                    Or.eval_binary_sets(
                        Set::from(vec![1, 2, 3]),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![4, 5, 6]).complement()
                );
                assert_eq!(
                    Or.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6])
                    ),
                    Set::from(vec![1, 2, 3]).complement()
                );
                assert_eq!(
                    Or.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![]).complement()
                );
            }
        }
    }
}
