use crate::node::Node;
use crate::operator::Operator;
use Node::*;
use Operator::*;

pub fn node_to_nnf(node: Node) -> Node {
    match node {
        UnaryExpr { .. } => unary_expr_to_nnf(node),
        BinaryExpr {
            op: op @ (And | Or),
            mut lhs,
            mut rhs,
        } => {
            *lhs = node_to_nnf(*lhs);
            *rhs = node_to_nnf(*rhs);
            BinaryExpr { op, lhs, rhs }
        }
        BinaryExpr { .. } => binary_expr_to_nnf(node),
        _ => node,
    }
}

pub fn binary_expr_to_nnf(node: Node) -> Node {
    if let BinaryExpr { op, lhs, rhs } = node {
        node_to_nnf(match op {
            Imply => !*lhs | *rhs,
            Xnor => (*lhs.clone() & *rhs.clone()) | (!*lhs & !*rhs),
            Xor => (!*lhs.clone() & *rhs.clone()) | (*lhs & !*rhs),
            _ => unreachable!(),
        })
    } else {
        node
    }
}

pub fn unary_expr_to_nnf(node: Node) -> Node {
    if let UnaryExpr { child, .. } = node {
        match *child {
            UnaryExpr { child, .. } => node_to_nnf(*child),

            BinaryExpr { op, lhs, rhs } => {
                let lhs = *lhs;
                let rhs = *rhs;
                node_to_nnf(match op {
                    Imply => !lhs | rhs,
                    Xnor => binary_expr_to_nnf(lhs ^ rhs),
                    And => !lhs | !rhs,
                    Or => !lhs & !rhs,
                    Xor => binary_expr_to_nnf(BinaryExpr {
                        op: Xnor,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }),
                    Not => unreachable!(),
                })
            }
            _ => UnaryExpr { op: Not, child },
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod nnf_test {
    use crate::nnf::{node_to_nnf, unary_expr_to_nnf};

    use crate::tree::Tree;
    use crate::truth_table::TruthTable;
    use std::str::FromStr;

    #[test]
    fn test_nnf_not_and() {
        let node = Tree::from_str("AB&!").unwrap();
        let result = Tree::from_str("A!B!|").unwrap();
        assert_eq!(node_to_nnf(node.root.clone()), result.root);
        assert_eq!(TruthTable::from(node), TruthTable::from(result));
    }

    #[test]
    fn test_nnf_imply() {
        let node = Tree::from_str("AB>").unwrap();
        let result = Tree::from_str("A!B|").unwrap();
        assert_eq!(node_to_nnf(node.root.clone()), result.root);
        assert_eq!(TruthTable::from(node), TruthTable::from(result));
    }

    #[test]
    fn test_nnf_xnor() {
        let node = Tree::from_str("AB=").unwrap();
        let result = Tree::from_str("AB&A!B!&|").unwrap();
        assert_eq!(node_to_nnf(node.root.clone()), result.root);
        assert_eq!(TruthTable::from(node), TruthTable::from(result));
    }

    #[test]
    fn test_nnf_xor() {
        let node = Tree::from_str("AB^").unwrap();
        let result = Tree::from_str("A!B&AB!&|").unwrap();
        assert_eq!(node_to_nnf(node.root.clone()), result.root);
        assert_eq!(TruthTable::from(node), TruthTable::from(result));
    }

    #[test]
    fn test_nnf_complex() {
        let node = Tree::from_str("AB|C&!").unwrap();
        let result = Tree::from_str("A!B!&C!|").unwrap();
        assert_eq!(node_to_nnf(node.root.clone()), result.root);
        assert_eq!(TruthTable::from(node), TruthTable::from(result));
    }

    #[test]
    fn test_nnf_not_or() {
        let node = Tree::from_str("AB|!").unwrap();
        let result = Tree::from_str("A!B!&").unwrap();
        assert_eq!(node_to_nnf(node.root.clone()), result.root);
        assert_eq!(TruthTable::from(node), TruthTable::from(result));
    }

    #[test]
    fn test_not_unary_odd() {
        let three = Tree::from_str("1!!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(three.root),
            Tree::from_str("1!").unwrap().root
        );
        let five = Tree::from_str("1!!!!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(five.root),
            Tree::from_str("1!").unwrap().root
        );
    }

    #[test]
    fn test_not_unary_even() {
        let two = Tree::from_str("1!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(two.root),
            Tree::from_str("1").unwrap().root
        );
        let four = Tree::from_str("1!!!!").unwrap();
        assert_eq!(
            unary_expr_to_nnf(four.root),
            Tree::from_str("1").unwrap().root
        );
    }
}
