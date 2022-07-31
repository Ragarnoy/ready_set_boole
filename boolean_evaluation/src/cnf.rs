use crate::nnf::node_to_nnf;
use crate::node::Node;
use crate::operator::Operator;
use Node::*;
use Operator::*;

pub fn node_to_cnf(node: Node) -> Node {
    let node = node_to_nnf(node);
    if let BinaryExpr { op, lhs, rhs } = node {
        match op {
            And => BinaryExpr {
                op: And,
                lhs: Box::new(node_to_cnf(*lhs)),
                rhs: Box::new(node_to_cnf(*rhs)),
            },
            Or => {
                let has_and = (
                    matches!(*lhs, BinaryExpr { op: And, .. }),
                    matches!(*rhs, BinaryExpr { op: And, .. }),
                );

                let lhs = node_to_cnf(*lhs);
                let rhs = node_to_cnf(*rhs);
                match has_and {
                    (true, true) => distribute_both(lhs | rhs),
                    (true, false) => distribute_to_left(lhs | rhs),
                    (false, true) => distribute_to_right(lhs | rhs),
                    _ => lhs | rhs,
                }
            }
            _ => unreachable!(),
        }
    } else {
        node
    }
}

// Distribute conjunctions over disjunctions.
fn distribute_both(node: Node) -> Node {
    if let BinaryExpr {
        op: Operator::Or,
        lhs,
        rhs,
    } = node
    {
        match (*lhs, *rhs) {
            (
                BinaryExpr {
                    op: And,
                    lhs: lhs_lhs,
                    rhs: lhs_rhs,
                },
                BinaryExpr {
                    op: And,
                    lhs: rhs_lhs,
                    rhs: rhs_rhs,
                },
            ) => {
                let lhs_lhs = node_to_cnf(*lhs_lhs);
                let lhs_rhs = node_to_cnf(*lhs_rhs);
                let rhs_lhs = node_to_cnf(*rhs_lhs);
                let rhs_rhs = node_to_cnf(*rhs_rhs);
                // ((ll | rl) & (ll | rr)) & ((lr | rl) & (lr | rr))
                ((lhs_lhs.clone() | rhs_lhs.clone()) & (lhs_lhs | rhs_rhs.clone()))
                    & ((lhs_rhs.clone() | rhs_lhs) & (lhs_rhs | rhs_rhs))
            }
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn distribute_to_left(node: Node) -> Node {
    if let BinaryExpr {
        op: Operator::Or,
        lhs,
        rhs,
    } = node
    {
        match *lhs {
            BinaryExpr {
                op: And,
                lhs: lhs_lhs,
                rhs: lhs_rhs,
            } => node_to_cnf(*lhs_lhs | *rhs.clone()) & node_to_cnf(*lhs_rhs | *rhs),
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn distribute_to_right(node: Node) -> Node {
    if let BinaryExpr { op: Or, lhs, rhs } = node {
        match *rhs {
            BinaryExpr {
                op: And,
                lhs: rhs_lhs,
                rhs: rhs_rhs,
            } => node_to_cnf(*lhs.clone() | *rhs_lhs) & node_to_cnf(*lhs | *rhs_rhs),
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod cnf_test {
    use crate::cnf::{distribute_both, distribute_to_left, distribute_to_right, node_to_cnf};
    use crate::tree::Tree;
    use std::str::FromStr;

    #[test]
    fn basic_node_to_cnf() {
        let node = Tree::from_str("ABC|DE&&|").unwrap();
        let result = Tree::from_str("ABC||AD|AE|&&").unwrap();
        assert_eq!(node_to_cnf(node.root), result.root);
    }

    #[test]
    fn basic_distribute_both() {
        let node = Tree::from_str("AB&CD&|").unwrap();
        let result = Tree::from_str("AC|AD|&BC|BD|&&").unwrap();
        assert_eq!(distribute_both(node.root), result.root);
    }

    #[test]
    fn basic_distribute_left() {
        let node = Tree::from_str("AB&C|").unwrap();
        let result = Tree::from_str("AC|BC|&").unwrap();
        assert_eq!(distribute_to_left(node.root), result.root);
    }

    #[test]
    fn basic_distribute_right() {
        let node = Tree::from_str("CAB&|").unwrap();
        let result = Tree::from_str("CA|CB|&").unwrap();
        assert_eq!(distribute_to_right(node.root), result.root);
    }
}
