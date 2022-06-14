use crate::nnf::node_to_negation_normal_form;
use crate::node::Node;
use crate::operator::Operator;

pub fn node_to_cnf(node: Node) -> Node {
    let nnf = node_to_negation_normal_form(node);
    match nnf {
        Node::UnaryExpr { .. } => node_to_cnf(nnf),
        Node::BinaryExpr {
            ref op,
            ref mut lhs,
            ref mut rhs,
        } => match *op {
            Operator::And | Operator::Or => {
                *lhs = Box::new(node_to_cnf(*lhs.clone()));
                *rhs = Box::new(node_to_cnf(*rhs.clone()));
                node.clone()
            },
            _ => binary_expr_to_cnf(node),
        },
        _ => nnf,
    }
}

fn cnf_handle_or(node: Node) -> {

}