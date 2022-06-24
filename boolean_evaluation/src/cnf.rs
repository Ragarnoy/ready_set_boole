use crate::nnf::node_to_negation_normal_form;
use crate::node::Node;
use crate::operator::Operator;

pub fn node_to_cnf(node: Node) -> Node {
    let mut nnf = node_to_negation_normal_form(node);
    match nnf {
        Node::UnaryExpr { .. } => node_to_cnf(nnf),
        Node::BinaryExpr {
            ref op,
            ref mut lhs,
            ref mut rhs,
        } => match *op {
            Operator::Or => {
                cnf_handle_or(*lhs.clone());
                cnf_handle_or(*rhs.clone());
                nnf.clone()
            },
            Operator::And => {
                cnf_handle_and(*lhs.clone());
                cnf_handle_and(*rhs.clone());
                nnf.clone()
            },
            _ => node_to_cnf(nnf),
        },
        _ => nnf,
    }
}

fn cnf_handle_or(node: Node) -> Node {
    todo!()
}

fn cnf_handle_and(node: Node) -> Node {
    todo!()
}
