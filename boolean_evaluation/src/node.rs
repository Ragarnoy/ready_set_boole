use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::operator::Operator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use crate::variable::Variable;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Variable(Rc<RefCell<Variable>>),
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
            Node::Variable(c) => ret.push(c.borrow().name),
            Node::Constant(x) => ret.push(if *x { '1' } else { '0' }),
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

        let mut node_stack: Vec<Node> = Vec::with_capacity(50);
        let mut vec_alphabet_ref: Rc<RefCell<Vec<Option<Rc<RefCell<Variable>>>>>> = Rc::new(RefCell::new(vec![None; 26]));
        let mut vec_alphabet: &mut Vec<Option<Rc<RefCell<Variable>>>> = vec_alphabet_ref.clone().get_mut();

        for c in s.chars() {
            let node = match c {
                'A'..='Z' => {
                    let idx = c as usize - 'A' as usize;
                    if let Some(v) = &vec_alphabet[idx] {
                        Node::Variable(v.clone())
                    } else {
                        let v = Rc::new(RefCell::new(Variable::new(c, false)));
                        vec_alphabet[idx] = Some(v.clone());
                        Node::Variable(v)
                    }
                },
                '1' => Node::Constant(true),
                '0' => Node::Constant(false),
                '&' => Node::BinaryExpr {
                    op: Operator::And,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '|' => Node::BinaryExpr {
                    op: Operator::Or,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '^' => Node::BinaryExpr {
                    op: Operator::Xor,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '>' => Node::BinaryExpr {
                    op: Operator::Imply,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '=' => Node::BinaryExpr {
                    op: Operator::Xnor,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '!' => Node::UnaryExpr {
                    op: Operator::Not,
                    child: Box::new(node_stack.pop().expect("Invalid input")),
                },
                _ => return Err("Invalid input".to_string()),
            };
            node_stack.push(node);
        }
        if node_stack.is_empty() {
            Err("Empty input".to_string())
        } else {
            Ok(node_stack.remove(0))
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

    pub fn print_rpn(&self) -> String {
        let mut ret = String::new();
        match self {
            // for a variable or constant, just print the value
            Node::Variable(c) => ret.push(c.borrow().name),
            Node::Constant(x) => ret.push(if *x { '1' } else { '0' }),
            // for a unary expression, first recurse on the child, then print the operator
            Node::UnaryExpr { op, child } => {
                ret.push_str(&*format!("{}{:?}", Node::print_rpn(child), op))
            }
            // for a binary expression, first recurse on the lhs, then recurse on the rhs, then print the operator
            Node::BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!(
                "{}{}{:?}",
                Node::print_rpn(lhs),
                Node::print_rpn(rhs),
                op
            )),
        }
        ret
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

#[cfg(test)]
mod node_tests {
    use crate::node::Node;
    use std::str::FromStr;

    #[test]
    // lots of rpn tests
    fn test_print_rpn() {
        let teststr = "AB&";
        let node = Node::from_str(teststr).unwrap();
        assert_eq!(node.print_rpn(), teststr);
    }

    #[test]
    fn test_print_rpn_2() {
        let teststr = "AB&C|";
        let node = Node::from_str(teststr).unwrap();
        assert_eq!(node.print_rpn(), teststr);
    }

    #[test]
    fn test_print_rpn_3() {
        let teststr = "AB&C|D^";
        let node = Node::from_str(teststr).unwrap();
        assert_eq!(node.print_rpn(), teststr);
    }

    #[test]
    fn test_print_rpn_4() {
        let teststr = "AB&C|D^!";
        let node = Node::from_str(teststr).unwrap();
        assert_eq!(node.print_rpn(), teststr);
    }
}
