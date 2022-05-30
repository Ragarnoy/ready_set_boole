mod nnf;

use crate::operator::Operator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::node::nnf::negation_normal_form;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Variable(char),
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
            Node::Variable(c) => ret.push(*c),
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
                'A'..='Z' => Node::Variable(c),
                '1' => Node::Constant(true),
                '0' => Node::Constant(false),
                '&' => Node::BinaryExpr {
                    op: Operator::And,
                    rhs: Box::new(node_vec.pop().expect("Invalid input")),
                    lhs: Box::new(node_vec.pop().expect("Invalid input")),
                },
                '|' => Node::BinaryExpr {
                    op: Operator::Or,
                    rhs: Box::new(node_vec.pop().expect("Invalid input")),
                    lhs: Box::new(node_vec.pop().expect("Invalid input")),
                },
                '^' => Node::BinaryExpr {
                    op: Operator::Xor,
                    rhs: Box::new(node_vec.pop().expect("Invalid input")),
                    lhs: Box::new(node_vec.pop().expect("Invalid input")),
                },
                '>' => Node::BinaryExpr {
                    op: Operator::Imply,
                    rhs: Box::new(node_vec.pop().expect("Invalid input")),
                    lhs: Box::new(node_vec.pop().expect("Invalid input")),
                },
                '=' => Node::BinaryExpr {
                    op: Operator::Xnor,
                    rhs: Box::new(node_vec.pop().expect("Invalid input")),
                    lhs: Box::new(node_vec.pop().expect("Invalid input")),
                },
                '!' => Node::UnaryExpr {
                    op: Operator::Not,
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
        match self {
            Node::Variable(_) => panic!("Variable node cannot be evaluated"),
            Node::Constant(p) => p,
            Node::BinaryExpr { op, lhs, rhs } => {
                eval_binary(lhs.compute_node(), op, rhs.compute_node())
            }
            Node::UnaryExpr { op, child } => eval_unary(op, child.compute_node()),
        }
    }

    pub fn nnf(self) -> Self {
        negation_normal_form(self)
    }

    pub fn child(&self) -> Option<&Node> {
        match self {
            Node::UnaryExpr { child, .. } => Some(child),
            _ => None,
        }
    }

    pub fn set_child(&mut self, child: Node) {
        match self {
            Node::UnaryExpr { child: _, .. } => {
                *self = Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(child),
                }
            }
            _ => panic!("Cannot set child for binary expression"),
        }
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, Node::Constant(_))
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Node::UnaryExpr { .. })
    }

    pub fn negation_normal_form(&mut self) {
        match self {
            Node::UnaryExpr { child, .. } => {
                if child.is_unary() {
                    child.negation_normal_form();
                }
            }

            Node::BinaryExpr { ref op, lhs, rhs } => {
                if lhs.is_unary() {
                    lhs.negation_normal_form();
                }
                if rhs.is_unary() {
                    rhs.negation_normal_form();
                }
                match *op {
                    Operator::Imply => {
                        *self = Node::BinaryExpr {
                            op: Operator::Or,
                            lhs: Box::new(Node::UnaryExpr {
                                op: Operator::Not,
                                child: Box::new(*lhs.clone()),
                            }),
                            rhs: Box::new(*rhs.clone()),
                        };
                    }
                    Operator::Xnor => {
                        *self = Node::BinaryExpr {
                            op: Operator::Or,
                            lhs: Box::new(Node::BinaryExpr {
                                op: Operator::And,
                                lhs: Box::new(Node::UnaryExpr {
                                    op: Operator::Not,
                                    child: Box::new(*lhs.clone()),
                                }),
                                rhs: Box::new(*rhs.clone()),
                            }),
                            rhs: Box::new(Node::BinaryExpr {
                                op: Operator::And,
                                lhs: Box::new(*lhs.clone()),
                                rhs: Box::new(Node::UnaryExpr {
                                    op: Operator::Not,
                                    child: Box::new(*rhs.clone()),
                                }),
                            }),
                        };
                    }
                    Operator::And => {}
                    Operator::Or => {}
                    Operator::Xor => {}
                    Operator::Not => {}
                }
            }
            _ => {}
        }
    }
}

fn eval_binary(lhs: bool, op: Operator, rhs: bool) -> bool {
    match op {
        Operator::Imply => !lhs | rhs,
        Operator::Xnor => lhs == rhs,
        Operator::And => lhs & rhs,
        Operator::Xor => lhs ^ rhs,
        Operator::Or => lhs | rhs,
        _ => unreachable!(),
    }
}

fn eval_unary(op: Operator, child: bool) -> bool {
    match op {
        Operator::Not => !child,
        _ => unreachable!(),
    }
}
