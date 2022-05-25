use crate::node::Node;
use crate::operator::Operator;

pub fn not_expr_to_nnf(node: &mut Node) -> Node {
    match node {
        Node::UnaryExpr { op, child } => not_expr_to_nnf(child),
        Node::BinaryExpr { op, lhs, rhs } => match *op {
            Operator::Imply => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(not_expr_to_nnf(lhs)),
                }),
                rhs: Box::new(not_expr_to_nnf(rhs)),
            },

            Operator::Xnor => Node::BinaryExpr {
                op: Operator::Xor,
                lhs: Box::new(not_expr_to_nnf(lhs)),
                rhs: Box::new(not_expr_to_nnf(rhs)),
            },

            Operator::And => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(not_expr_to_nnf(lhs)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(not_expr_to_nnf(rhs)),
                }),
            },

            Operator::Or => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(not_expr_to_nnf(lhs)),
                }),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(not_expr_to_nnf(rhs)),
                }),
            },

            Operator::Xor => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(not_expr_to_nnf(lhs)),
                    }),
                    rhs: Box::new(not_expr_to_nnf(rhs)),
                }),
                rhs: Box::new(Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(not_expr_to_nnf(lhs)),
                    rhs: Box::new(Node::UnaryExpr {
                        op: Operator::Not,
                        child: Box::new(not_expr_to_nnf(rhs)),
                    }),
                }),
            },
            _ => {
                unreachable!()
            }
        },
    }
}
