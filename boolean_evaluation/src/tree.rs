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

#[derive(Debug, Clone)]
pub struct Tree {
    pub root: Node,
    universe: Vec<i32>,
    pub variable_list: Option<VariableRefList>,
}

impl Tree {
    pub fn sat(self) -> bool {
        if let Some(variable_list) = self.variable_list {
            for bitfield in 0..2u32.pow(variable_list.len() as u32) {
                for (i, v) in variable_list.iter().filter(|v| v.is_some()).enumerate() {
                    {
                        let mut v = v.as_ref().unwrap().borrow_mut();
                        v.value = (bitfield & (1u32 << i)) != 0;
                    }
                    if self.root.eval_ref() {
                        return true;
                    }
                }
            }
            false
        } else {
            false
        }
    }

    pub fn evaluate_sets(self) -> Vec<i32> {
        let ret = self.root.compute_sets();
        if ret.is_complement {
            self.universe
                .iter()
                .filter(|x| !ret.values.contains(x)).copied()
                .collect()
        } else {
            ret.values
        }
    }

    pub fn assign_sets(&mut self, sets: Vec<Vec<i32>>) {
        if let Some(variable_list) = &self.variable_list {
            if sets.len() != variable_list.len() {
                panic!("Number of sets does not match number of variables");
            }
            self.universe = sets.iter().flatten().copied().collect();
            self.universe.sort_unstable();
            self.universe.dedup();
            sets.into_iter()
                .zip(variable_list.iter())
                .filter(|(_, v)| v.is_some())
                .for_each(|(set, v)| {
                    let mut v = v.as_ref().unwrap().borrow_mut();
                    v.set = Some(set);
                });
        }
    }
}

impl FromStr for Tree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() && !s.contains(VALID_TOKENS) {
            return Err(String::from("Empty input!"));
        }

        let mut node_stack: Vec<Node> = Vec::with_capacity(50);
        let mut vec_variables: VariableRefList = vec![None; 26];

        for c in s.chars() {
            let node = match c {
                'A'..='Z' => {
                    let idx = c as usize - 'A' as usize;
                    if let Some(v) = &vec_variables[idx] {
                        Variable(v.clone())
                    } else {
                        let v = Rc::new(RefCell::new(Variable::new(c)));
                        vec_variables[idx] = Some(v.clone());
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
                universe: vec![],
                variable_list: if s.contains(char::is_alphabetic) {
                    vec_variables.retain(|v| v.is_some());
                    Some(vec_variables)
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

    mod set_tests {
        use crate::tree::Tree;
        use std::str::FromStr;
        #[test]
        fn test_set_false() {
            let mut tree = Tree::from_str("AB&").unwrap();
            tree.assign_sets(vec![vec![1, 2], vec![3, 4]]);
            assert_eq!(tree.evaluate_sets(), vec![]);
            let mut tree = Tree::from_str("A!").unwrap();
            tree.assign_sets(vec![vec![1, 2, 3]]);
            assert_eq!(tree.evaluate_sets(), vec![]);
        }

        #[test]
        fn test_set_true() {
            let mut tree = Tree::from_str("AB|").unwrap();
            tree.assign_sets(vec![vec![0, 1], vec![2, 3]]);
            assert_eq!(tree.evaluate_sets(), vec![0, 1, 2, 3]);
        }
    }
}
