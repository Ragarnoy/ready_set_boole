use std::fmt::{Display, Formatter};
use core::fmt;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

#[derive(Debug, Clone)]
pub enum Operator {
    Neg,
    And,
    Or,
    Xor,
    Imply,
    Leq,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Neg => write!(f, "¬"),
            Operator::And => write!(f, "∧"),
            Operator::Or => write!(f, "∨"),
            Operator::Xor => write!(f, "⊕"),
            Operator::Imply => write!(f, "→"),
            Operator::Leq => write!(f, "⇔"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Val(bool),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();

        match self {
            Node::Val(x) => ret.push_str(&*(if *x { "⊤" } else { "⊥" }).to_string()),
            Node::UnaryExpr { op, child } => ret.push_str(&*format!("{}{}", op, child)),
            Node::BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!("{} {} {}", lhs, op, rhs)),
        }
        write!(f, "{}", ret)
    }
}

fn parse_string(str: &str) -> Node {
    let mut node_vec: Vec<Node> = Vec::with_capacity(50);

    for c in str.chars() {
        let node = match c {
            '1' => Node::Val(true),
            '0' => Node::Val(false),
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
        Node::Val(p) => p,
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
    println!("{}", res);
    res
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
