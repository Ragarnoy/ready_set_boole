use crate::nnf::node_to_negation_normal_form;
use crate::node::Node;
use crate::operator::Operator;

pub fn node_to_cnf(node: Node) -> Node {
    if let Node::BinaryExpr { op, lhs, rhs } = node_to_negation_normal_form(node.clone()) {
        match op {
            Operator::And => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(node_to_cnf(*lhs)),
                rhs: Box::new(node_to_cnf(*rhs)),
            },
            Operator::Or => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(node_to_cnf(*lhs)),
                rhs: Box::new(node_to_cnf(*rhs)),
            },
            _ => unreachable!(),
        }
    } else {
        node
    }
}

// Distribute conjunctions over disjunctions.
fn distribute(factor: Node, node: Node) -> Node {
    todo!()
}
