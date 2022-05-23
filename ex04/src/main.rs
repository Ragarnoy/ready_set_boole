mod node;
mod operator;
mod tree;
mod truth_table;

use crate::node::Node;
use crate::operator::Operator;
use std::str::FromStr;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

fn eval_binary(lhs: bool, op: Operator, rhs: bool) -> bool {
    match op {
        Operator::Imply => !(!lhs & rhs),
        Operator::Leq => lhs == rhs,
        Operator::And => lhs & rhs,
        Operator::Xor => lhs ^ rhs,
        Operator::Or => lhs | rhs,
        _ => unreachable!(),
    }
}

fn eval_unary(op: Operator, child: bool) -> bool {
    match op {
        Operator::Neg => !child,
        _ => unreachable!(),
    }
}

fn compute_node(node: Node) -> bool {
    let current = node;

    match current {
        Node::Constant(p) => p,
        Node::BinaryExpr { op, lhs, rhs } => {
            eval_binary(compute_node(*lhs), op, compute_node(*rhs))
        }
        Node::UnaryExpr { op, child } => eval_unary(op, compute_node(*child)),
    }
}

fn calculate_truth_table(formula: &str) {
    if formula.is_empty() {
        panic!("Empty input!");
    }
    if !formula.contains(VALID_TOKENS) || !formula.contains(|ch| ('A'..='Z').contains(&ch)) {
        panic!("Invalid tokens");
    }

    let variables: Vec<(usize, char)> = formula
        .to_string()
        .char_indices()
        .into_iter()
        .filter(|(_, ch)| ch.is_ascii_uppercase())
        .collect();

    let mut nodes: Vec<Node> = Vec::with_capacity(variables.len().pow(2));
    let mut char_vec = formula.to_string().chars().collect::<Vec<char>>();
    for bitfield in 0..2_u32.pow(variables.len() as u32) {
        for i in 0..variables.len() {
            char_vec[variables[i].0] = if bitfield & (1 << i) != 0 { '1' } else { '0' };
        }
        nodes.push(Node::from_str(&char_vec.iter().collect::<String>()).unwrap());
    }

    for node in nodes {
        println!("{} -> {}", node.clone(), compute_node(node));
    }
}

fn print_truth_table(nodes: Vec<Node>, map: Vec<(usize, char)>) {
    let mut fline: String = map.iter().map(|(_, c)| format!("| {} |", c)).collect();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        calculate_truth_table(&args[1]);
    } else {
        calculate_truth_table("AB&C|");
    }
}

#[cfg(test)]
mod test_truth_table {
}
