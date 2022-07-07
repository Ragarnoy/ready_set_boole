use crate::node::Node;
use crate::operator::Operator;
use crate::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use Node::*;
use Operator::*;

pub type VariableRef = Rc<RefCell<Variable>>;
pub type VariableRefList = Vec<Option<VariableRef>>;
const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

pub struct Tree {
    pub root: Node,
    pub variable_list: Option<VariableRefList>,
}

impl FromStr for Tree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(String::from("Empty input!"));
        }
        if !s.contains(VALID_TOKENS) {
            return Err(String::from("Invalid tokens"));
        }

        let mut node_stack: Vec<Node> = Vec::with_capacity(50);
        let mut vec_alphabet: VariableRefList = vec![None; 26];

        for c in s.chars() {
            let node = match c {
                'A'..='Z' => {
                    let idx = c as usize - 'A' as usize;
                    if let Some(v) = &vec_alphabet[idx] {
                        Variable(v.clone())
                    } else {
                        let v = Rc::new(RefCell::new(Variable::new(c, false)));
                        vec_alphabet[idx] = Some(v.clone());
                        Variable(v)
                    }
                }
                '1' => Constant(true),
                '0' => Constant(false),
                '&' => BinaryExpr {
                    op: And,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '|' => BinaryExpr {
                    op: Or,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '^' => BinaryExpr {
                    op: Xor,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '>' => BinaryExpr {
                    op: Imply,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '=' => BinaryExpr {
                    op: Xnor,
                    rhs: Box::new(node_stack.pop().expect("Invalid input")),
                    lhs: Box::new(node_stack.pop().expect("Invalid input")),
                },
                '!' => UnaryExpr {
                    op: Not,
                    child: Box::new(node_stack.pop().expect("Invalid input")),
                },
                _ => return Err("Invalid input".to_string()),
            };
            node_stack.push(node);
        }
        if node_stack.len() != 1 {
            Err("Invalid input".to_string())
        } else {
            Ok(Self {
                root: node_stack.remove(0),
                variable_list: if s.contains(char::is_alphabetic) {
                    Some(vec_alphabet)
                } else {
                    None
                },
            })
        }
    }
}
