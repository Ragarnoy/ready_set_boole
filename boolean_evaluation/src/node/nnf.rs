use crate::node::Node;
use crate::operator::Operator;

pub fn not_binary_expr_to_nnf(node: Node) -> Node {
    if let Node::BinaryExpr { op, lhs, rhs } = node {
        match op {
            Operator::Imply => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: lhs,
                }),
                rhs,
            },

            Operator::Xnor => Node::BinaryExpr {
                op: Operator::Xor,
                lhs,
                rhs,
            },

            Operator::And => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: lhs,
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: rhs,
                }),
            },

            Operator::Or => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: lhs,
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: rhs,
                }),
            },

            Operator::Xor => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(Node::UnaryExpr {
                        op: Operator::Not,
                        child: lhs.clone(),
                    }),
                    rhs: rhs.clone(),
                }),
                rhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs,
                    rhs: Box::new(Node::UnaryExpr {
                        op: Operator::Not,
                        child: rhs,
                    }),
                }),
            },
            Operator::Not => {
                unreachable!();
            }
        }
    } else {
        node
    }
}

pub fn not_unary_expr_to_nnf(node: Node) -> Node {
    match node {
        Node::UnaryExpr { child, .. } => match *child {
            Node::UnaryExpr { child, .. } => *child,
            _ => not_unary_expr_to_nnf(*child),
        },
        _ => node,
    }
}

#[cfg(test)]
mod nnf_test {
    use crate::node::nnf::{not_binary_expr_to_nnf, not_unary_expr_to_nnf};
    use crate::node::Node;
    use std::str::FromStr;

    #[test]
    fn test_not_unary_odd() {
        let three = Node::from_str("1!!!").unwrap();
        assert_eq!(not_unary_expr_to_nnf(three), Node::from_str("1!").unwrap());
        let five = Node::from_str("1!!!!!").unwrap();
        assert_eq!(not_unary_expr_to_nnf(five), Node::from_str("1!").unwrap());
    }

    #[test]
    fn test_not_unary_even() {
        let two = Node::from_str("1!!").unwrap();
        assert_eq!(not_unary_expr_to_nnf(two), Node::from_str("1").unwrap());
        let four = Node::from_str("1!!!!").unwrap();
        assert_eq!(not_unary_expr_to_nnf(four), Node::from_str("1").unwrap());
    }

    #[test]
    fn test_not_binary_expr() {
        let and: Node = Node::from_str("11&").unwrap();
        let not_or = Node::from_str("11|!").unwrap();
        let not_xor = Node::from_str("11^!").unwrap();
        assert_eq!(
            not_binary_expr_to_nnf(and),
            Node::from_str("1!1!|").unwrap()
        );
    }
}
