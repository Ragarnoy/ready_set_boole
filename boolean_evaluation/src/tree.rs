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

impl Tree {
    pub fn sat(self) -> bool {
        if let Some(variable_list) = self.variable_list {
            for bitfield in 0..2u32.pow(variable_list.iter().filter(|v| v.is_some()).count() as u32)
            {
                for (i, v) in variable_list.iter().filter(|v| v.is_some()).enumerate() {
                    {
                        let mut v = v.as_ref().unwrap().borrow_mut();
                        v.value = (bitfield & (1u32 << i)) != 0;
                    }
                    if self.root.compute_node() {
                        return true;
                    }
                }
            }
            false
        } else {
            false
        }
    }
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
                        let v = Rc::new(RefCell::new(Variable::new(c)));
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

#[cfg(test)]
mod tree_tests {

    mod sat_tests {
        use crate::tree::Tree;
        use std::str::FromStr;

        #[test]
        fn test_sat_false() {
            let tree = Tree::from_str("AA^").unwrap();
            assert!(!tree.sat());
        }

        #[test]
        fn test_sat_true() {
            let tree = Tree::from_str("AB|").unwrap();
            assert!(tree.sat());
        }
    }
}
