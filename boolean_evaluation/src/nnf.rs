use crate::node::Node;
use crate::operator::Operator;

pub fn node_to_negation_normal_form(node: Node) -> Node {
    match node {
        Node::UnaryExpr { .. } => unary_expr_to_nnf(node),
        Node::BinaryExpr {
            op: op @ (Operator::And | Operator::Or),
            mut lhs,
            mut rhs,
        } => {
            *lhs = node_to_negation_normal_form(*lhs);
            *rhs = node_to_negation_normal_form(*rhs);
            Node::BinaryExpr { op, lhs, rhs }
        }
        Node::BinaryExpr { .. } => binary_expr_to_nnf(node),
        _ => node,
    }
}

pub fn binary_expr_to_nnf(node: Node) -> Node {
    if let Node::BinaryExpr { op, lhs, rhs } = node {
        match op {
            Operator::Imply => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(node_to_negation_normal_form(*lhs)),
                }),
                rhs: Box::new(node_to_negation_normal_form(*rhs)),
            },
            Operator::Xnor => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(node_to_negation_normal_form(*lhs.clone())),
                    rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                }),
                rhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(node_to_negation_normal_form(*lhs)),
                    }),
                    rhs: Box::new(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(node_to_negation_normal_form(*rhs)),
                    }),
                }),
            },
            Operator::Xor => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                }),
                rhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(node_to_negation_normal_form(*lhs)),
                    rhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*rhs),
                    })),
                }),
            },
            _ => unreachable!(),
        }
    } else {
        node
    }
}

pub fn unary_expr_to_nnf(node: Node) -> Node {
    if let Node::UnaryExpr { child, .. } = node {
        match *child {
            Node::UnaryExpr { child, .. } => node_to_negation_normal_form(*child),
            Node::BinaryExpr {
                op,
                lhs,
                rhs,
            } => match op {
                Operator::Imply => Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(*rhs)),
                },

                Operator::Xnor => Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(Node::BinaryExpr {
                        op: Operator::And,
                        lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                            op: Operator::Not,
                            child: Box::new(*lhs.clone()),
                        })),
                        rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                    }),
                    rhs: Box::new(Node::BinaryExpr {
                        op: Operator::And,
                        lhs: Box::new(node_to_negation_normal_form(*lhs)),
                        rhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                            op: Operator::Not,
                            child: Box::new(*rhs),
                        })),
                    }),
                },

                Operator::And => Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*rhs),
                    })),
                },

                Operator::Or => Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*rhs),
                    })),
                },

                Operator::Xor => Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(node_to_negation_normal_form(Node::BinaryExpr { 
                        op: Operator::And,
                        lhs: Box::new(Node::UnaryExpr {
                            op: Operator::Not,
                            child: Box::new(*lhs.clone()),
                        }),
                        rhs: Box::new(*rhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(Node::BinaryExpr {
                        op: Operator::And,
                        lhs: Box::new(*lhs),
                        rhs: Box::new(Node::UnaryExpr {
                            op: Operator::Not,
                            child: Box::new(*rhs),
                        }),
                    })),
                },
                Operator::Not => {
                    unreachable!();
                }
            },
            _ => Node::UnaryExpr {
                op: Operator::Not,
                child,
            },
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod nnf_test {
    use crate::nnf::{node_to_negation_normal_form, unary_expr_to_nnf};
    use crate::node::Node;
    use std::str::FromStr;

    #[test]
    fn test_nnf_not_and() {
        let node = Node::from_str("AB&!").unwrap();
        let result = Node::from_str("A!B!|").unwrap();
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_nnf_imply() {
        let node = Node::from_str("AB>").unwrap();
        let result = Node::from_str("A!B|").unwrap();
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_nnf_xnor() {
        let node = Node::from_str("AB=").unwrap();
        let result = Node::from_str("AB&A!B!&|").unwrap();
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_nnf_xor() {
        let node = Node::from_str("AB^").unwrap();
        let result = Node::from_str("A!B&AB!&|").unwrap();
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_nnf_complex() {
        let node = Node::from_str("AB|C&!").unwrap();
        let result = Node::from_str("A!B!&C!|").unwrap();
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_nnf_not_or() {
        let node = Node::from_str("AB|!").unwrap();
        let result = Node::from_str("A!B!&").unwrap();
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_not_unary_odd() {
        let three = Node::from_str("1!!!").unwrap();
        assert_eq!(unary_expr_to_nnf(three), Node::from_str("1!").unwrap());
        let five = Node::from_str("1!!!!!").unwrap();
        assert_eq!(unary_expr_to_nnf(five), Node::from_str("1!").unwrap());
    }

    #[test]
    fn test_not_unary_even() {
        let two = Node::from_str("1!!").unwrap();
        assert_eq!(unary_expr_to_nnf(two), Node::from_str("1").unwrap());
        let four = Node::from_str("1!!!!").unwrap();
        assert_eq!(unary_expr_to_nnf(four), Node::from_str("1").unwrap());
    }
}
