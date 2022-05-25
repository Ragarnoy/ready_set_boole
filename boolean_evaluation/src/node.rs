use crate::operator::Operator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

#[derive(Debug, Clone)]
pub enum Node {
    Constant(bool),
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
            Node::Constant(x) => ret.push_str(&*(if *x { "1" } else { "0" }).to_string()),
            Node::UnaryExpr { op, child } => ret.push_str(&*format!("{}{}", op, child)),
            Node::BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!("{} {} {}", lhs, op, rhs)),
        }
        write!(f, "{}", ret)
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(String::from("Empty input!"));
        }
        if !s.contains(VALID_TOKENS) {
            return Err(String::from("Invalid tokens"));
        }

        let mut node_vec: Vec<Node> = Vec::with_capacity(50);

        for c in s.chars() {
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
                _ => return Err("Invalid input".to_string()),
            };
            node_vec.push(node);
        }
        if node_vec.is_empty() {
            Err("Empty input".to_string())
        } else {
            Ok(node_vec.remove(0))
        }
    }
}

impl Node {
    pub fn compute_node(self) -> bool {
        let current = self;

        match current {
            Node::Constant(p) => p,
            Node::BinaryExpr { op, lhs, rhs } => {
                eval_binary(lhs.compute_node(), op, rhs.compute_node())
            }
            Node::UnaryExpr { op, child } => eval_unary(op, child.compute_node()),
        }
    }
}

fn eval_binary(lhs: bool, op: Operator, rhs: bool) -> bool {
    match op {
        Operator::Imply => !lhs | rhs,
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
