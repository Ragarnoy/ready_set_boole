use crate::set::Set;
use std::fmt;
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
            Operator::Not => write!(f, "¬"),
            Operator::And => write!(f, "∧"),
            Operator::Or => write!(f, "∨"),
            Operator::Xor => write!(f, "⊕"),
            Operator::Imply => write!(f, "→"),
            Operator::Xnor => write!(f, "⇔"),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Not => write!(f, "!"),
            Operator::And => write!(f, "&"),
            Operator::Or => write!(f, "|"),
            Operator::Xor => write!(f, "^"),
            Operator::Imply => write!(f, ">"),
            Operator::Xnor => write!(f, "="),
        }
    }
}

impl Operator {
    pub fn eval_binary(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Operator::Imply => !lhs | rhs,
            Operator::Xnor => lhs == rhs,
            Operator::And => lhs & rhs,
            Operator::Xor => lhs ^ rhs,
            Operator::Or => lhs | rhs,
            _ => unreachable!(),
        }
    }

    pub fn eval_unary(self, child: bool) -> bool {
        match self {
            Operator::Not => !child,
            _ => unreachable!(),
        }
    }

    pub fn eval_binary_sets(self, lhs: Set, rhs: Set) -> Set {
        match self {
            Operator::And => match (lhs.is_complement, rhs.is_complement) {
                (false, false) => lhs
                    .values
                    .iter()
                    .filter(|x| rhs.values.contains(x))
                    .copied()
                    .collect(),
                (false, true) => lhs
                    .values
                    .iter()
                    .filter(|x| !rhs.values.contains(x))
                    .copied()
                    .collect(),
                (true, false) => rhs
                    .values
                    .iter()
                    .filter(|x| !lhs.values.contains(x))
                    .copied()
                    .collect(),
                (true, true) => Set {
                    values: [
                        lhs.values.clone(),
                        rhs.values
                            .iter()
                            .filter(|x| !lhs.values.contains(x))
                            .copied()
                            .collect(),
                    ]
                    .concat(),
                    is_complement: true,
                },
            },
            Operator::Or => match (lhs.is_complement, rhs.is_complement) {
                (false, false) => [
                    lhs.values.clone(),
                    rhs.values
                        .iter()
                        .filter(|x| !lhs.values.contains(x))
                        .copied()
                        .collect(),
                ]
                .concat()
                .into(),
                (false, true) => lhs,
                (true, false) => rhs,
                (true, true) => Set {
                    values: lhs
                        .values
                        .iter()
                        .filter(|x| rhs.values.contains(x))
                        .copied()
                        .collect(),
                    is_complement: true,
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn eval_unary_sets(self, mut child: Set) -> Set {
        match self {
            Operator::Not => {
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
                assert!(Operator::And.eval_binary(true, true));
                assert!(!Operator::And.eval_binary(true, false));
                assert!(!Operator::And.eval_binary(false, true));
                assert!(!Operator::And.eval_binary(false, false));
            }

            #[test]
            fn test_eval_or() {
                assert!(Operator::Or.eval_binary(true, true));
                assert!(Operator::Or.eval_binary(true, false));
                assert!(Operator::Or.eval_binary(false, true));
                assert!(!Operator::Or.eval_binary(false, false));
            }

            #[test]
            fn test_eval_xor() {
                assert!(!Operator::Xor.eval_binary(true, true));
                assert!(Operator::Xor.eval_binary(true, false));
                assert!(Operator::Xor.eval_binary(false, true));
                assert!(!Operator::Xor.eval_binary(false, false));
            }

            #[test]
            fn test_eval_imply() {
                assert!(Operator::Imply.eval_binary(true, true));
                assert!(!Operator::Imply.eval_binary(true, false));
                assert!(Operator::Imply.eval_binary(false, true));
                assert!(Operator::Imply.eval_binary(false, false));
            }

            #[test]
            fn test_eval_xnor() {
                assert!(Operator::Xnor.eval_binary(true, true));
                assert!(!Operator::Xnor.eval_binary(true, false));
                assert!(!Operator::Xnor.eval_binary(false, true));
                assert!(Operator::Xnor.eval_binary(false, false));
            }
        }
        mod unary {
            use super::*;

            #[test]
            fn test_eval_unary() {
                assert!(!Operator::Not.eval_unary(true));
                assert!(Operator::Not.eval_unary(false));
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
                    Operator::And
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![2, 3, 4])),
                    Set::from(vec![2, 3])
                );
                assert_eq!(
                    Operator::And
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![4, 5, 6])),
                    Set::from(vec![])
                );
                assert_eq!(
                    Operator::And.eval_binary_sets(
                        Set::from(vec![1, 2, 3]),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![1, 2, 3])
                );
                assert_eq!(
                    Operator::And.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6])
                    ),
                    Set::from(vec![4, 5, 6])
                );
                assert_eq!(
                    Operator::And.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![1, 2, 3, 4, 5, 6]).complement()
                );
            }

            #[test]
            fn test_eval_or() {
                assert_eq!(
                    Operator::Or
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![2, 3, 4])),
                    Set::from(vec![1, 2, 3, 4])
                );
                assert_eq!(
                    Operator::Or
                        .eval_binary_sets(Set::from(vec![1, 2, 3]), Set::from(vec![4, 5, 6])),
                    Set::from(vec![1, 2, 3, 4, 5, 6])
                );
                assert_eq!(
                    Operator::Or.eval_binary_sets(
                        Set::from(vec![1, 2, 3]),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![1, 2, 3])
                );
                assert_eq!(
                    Operator::Or.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6])
                    ),
                    Set::from(vec![4, 5, 6])
                );
                assert_eq!(
                    Operator::Or.eval_binary_sets(
                        Set::from(vec![1, 2, 3]).complement(),
                        Set::from(vec![4, 5, 6]).complement()
                    ),
                    Set::from(vec![1, 2, 3, 4, 5, 6])
                );
            }
        }
    }
}
