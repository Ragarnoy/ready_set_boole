use boolean_evaluation::node::Node;
use boolean_evaluation::operator::Operator;

pub fn node_to_negation_normal_form(mut node: Node) -> Node {
    match node {
        Node::UnaryExpr { .. } => unary_expr_to_nnf(node),

        Node::BinaryExpr {
            ref op,
            ref mut lhs,
            ref mut rhs,
        } => match *op {
            Operator::Imply => binary_expr_to_nnf(node),
            Operator::Xnor => binary_expr_to_nnf(node),
            _ => {
                *lhs = Box::new(node_to_negation_normal_form(*lhs.clone()));
                *rhs = Box::new(node_to_negation_normal_form(*rhs.clone()));
                node.clone()
            }
        },
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
            _ => unreachable!(),
        }
    } else {
        node
    }
}

pub fn unary_expr_to_nnf(node: Node) -> Node {
    match node {
        Node::UnaryExpr { ref child, .. } => match &**child {
            Node::UnaryExpr { ref child, .. } => node_to_negation_normal_form(*child.clone()),
            Node::BinaryExpr {
                ref op,
                ref lhs,
                ref rhs,
            } => match op {
                Operator::Imply => Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                },

                Operator::Xnor => Node::BinaryExpr {
                    op: Operator::Xor,
                    lhs: Box::new(node_to_negation_normal_form(*lhs.clone())),
                    rhs: Box::new(node_to_negation_normal_form(*rhs.clone())),
                },

                Operator::And => Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*rhs.clone()),
                    })),
                },

                Operator::Or => Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*lhs.clone()),
                    })),
                    rhs: Box::new(node_to_negation_normal_form(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(*rhs.clone()),
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
                        lhs: Box::new(*lhs.clone()),
                        rhs: Box::new(Node::UnaryExpr {
                            op: Operator::Not,
                            child: Box::new(*rhs.clone()),
                        }),
                    })),
                },
                Operator::Not => {
                    unreachable!();
                }
            },
            Node::Variable(_) => node,
            Node::Constant(_) => node,
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod nnf_test {
    use crate::nnf::{node_to_negation_normal_form, unary_expr_to_nnf};
    use boolean_evaluation::node::Node;
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
        println!("{}", node_to_negation_normal_form(node.clone()));
        assert_eq!(node_to_negation_normal_form(node), result);
    }

    #[test]
    fn test_nnf_xnor() {
        let node = Node::from_str("AB=").unwrap();
        let result = Node::from_str("AB&A!B!&|").unwrap();
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
