mod node;
mod operator;
mod tree;

use crate::node::Node;
use crate::operator::Operator;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

fn parse_string(str: &str) -> Node {
    let mut node_vec: Vec<Node> = Vec::with_capacity(50);

    for c in str.chars() {
        let node = match c {
            '1' => Node::Constant(true),
            '0' => Node::Constant(false),
            '&' => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(node_vec.pop().expect("Invalid input")),
                rhs: Box::new(node_vec.pop().expect("Invalid input")),
            },
            '|' => Node::BinaryExpr {
                op: Operator::Or,
                lhs: Box::new(node_vec.pop().expect("Invalid input")),
                rhs: Box::new(node_vec.pop().expect("Invalid input")),
            },
            '^' => Node::BinaryExpr {
                op: Operator::Xor,
                lhs: Box::new(node_vec.pop().expect("Invalid input")),
                rhs: Box::new(node_vec.pop().expect("Invalid input")),
            },
            '>' => Node::BinaryExpr {
                op: Operator::Imply,
                lhs: Box::new(node_vec.pop().expect("Invalid input")),
                rhs: Box::new(node_vec.pop().expect("Invalid input")),
            },
            '=' => Node::BinaryExpr {
                op: Operator::Leq,
                lhs: Box::new(node_vec.pop().expect("Invalid input")),
                rhs: Box::new(node_vec.pop().expect("Invalid input")),
            },
            '!' => Node::UnaryExpr {
                op: Operator::Neg,
                child: Box::new(node_vec.pop().expect("Invalid input")),
            },
            _ => panic!("Invalid input"),
        };
        node_vec.push(node);
    }
    if node_vec.is_empty() {
        panic!("No tokens to eval");
    }
    node_vec.remove(0)
}

fn eval_binary(lhs: bool, op: Operator, rhs: bool) -> bool {
    match op {
        Operator::Imply => if lhs && rhs { true } else if !lhs && rhs { true } else { !lhs && !rhs },
        Operator::Leq => lhs == rhs,
        Operator::And => lhs & rhs,
        Operator::Xor => lhs ^ rhs,
        Operator::Or => lhs | rhs,
        _ => panic!("Impossible"),
    }
}

fn eval_unary(op: Operator, child: bool) -> bool {
    match op {
        Operator::Neg => !child,
        _ => panic!("Impossible"),
    }
}

fn compute_node(node: Node) -> bool {
    let current = node;

    match current {
        Node::Constant(p) => p,
        Node::BinaryExpr { op, lhs, rhs } => eval_binary(compute_node(*lhs), op, compute_node(*rhs)),
        Node::UnaryExpr { op, child } => eval_unary(op, compute_node(*child)),
    }
}

fn print_truth_table(formula: &str) -> bool {
    if formula.is_empty() {
        panic!("Empty input!");
    }
    if !formula.contains(VALID_TOKENS) || !formula.contains(|ch| ('A'..='Z').contains(&ch)) {
        panic!("Invalid tokens");
    }

    for (j, c) in formula.to_string().char_indices() {

    }
    let node = parse_string(formula);
    let res = compute_node(node);
    res
}

fn cartesian_product(repeat: usize) {
    for (a, b) in (0..4).map(|n| (n & 2 != 0, n & 1 != 0))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        print_truth_table(&args[1]);
    } else {
        print_truth_table("10=");
    }
}

#[cfg(test)]
mod test_truth_table {
    use super::*;
    use std::panic;
}
