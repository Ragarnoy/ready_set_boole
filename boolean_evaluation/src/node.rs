use crate::operator::Operator;
use crate::set::Set;
use crate::variable::Variable;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::{fmt, ops};
use Node::*;
use Operator::*;

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
            Variable(c) => ret.push(RefCell::borrow(c).name),
            Constant(x) => ret.push(if *x { '1' } else { '0' }),
            UnaryExpr { op, child } => ret.push_str(&*format!("{}{}", op, child)),
            BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!("{} {} {}", lhs, op, rhs)),
        }
        write!(f, "{}", ret)
    }
}

impl Node {
    pub fn eval(self) -> bool {
        match self {
            Variable(v) => v.borrow().value,
            Constant(p) => p,
            BinaryExpr { op, lhs, rhs } => eval_binary(lhs.eval(), op, rhs.eval()),
            UnaryExpr { op, child } => eval_unary(op, child.eval()),
        }
    }

    pub fn eval_ref(&self) -> bool {
        match self {
            Variable(v) => v.borrow().value,
            Constant(p) => *p,
            BinaryExpr { op, lhs, rhs } => eval_binary(lhs.eval_ref(), *op, rhs.eval_ref()),
            UnaryExpr { op, child } => eval_unary(*op, child.eval_ref()),
        }
    }

    pub fn compute_sets(&self) -> Set {
        match self {
            Variable(v) => Set::new(v.borrow().set.as_ref().unwrap().clone()),
            Constant(p) => Set::new(vec![if *p { 1 } else { 0 }]),
            BinaryExpr { op, lhs, rhs } => {
                let lhs_sets = lhs.compute_sets();
                let rhs_sets = rhs.compute_sets();
                eval_binary_sets(lhs_sets, *op, rhs_sets)
            }
            UnaryExpr { op, child } => {
                let child_sets = child.compute_sets();
                eval_unary_sets(*op, child_sets)
            }
        }
    }

    pub fn print_rpn(&self) -> String {
        let mut ret = String::new();
        match self {
            // for a variable or constant, just print the value
            Variable(c) => ret.push(RefCell::borrow(c).name),
            Constant(x) => ret.push(if *x { '1' } else { '0' }),
            // for a unary expression, first recurse on the child, then print the operator
            UnaryExpr { op, child } => {
                ret.push_str(&*format!("{}{:?}", Self::print_rpn(child), op))
            }
            // for a binary expression, first recurse on the lhs, then recurse on the rhs, then print the operator
            BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!(
                "{}{}{:?}",
                Self::print_rpn(lhs),
                Self::print_rpn(rhs),
                op
            )),
        }
        ret
    }
}

fn eval_binary(lhs: bool, op: Operator, rhs: bool) -> bool {
    match op {
        Imply => !lhs | rhs,
        Xnor => lhs == rhs,
        And => lhs & rhs,
        Xor => lhs ^ rhs,
        Or => lhs | rhs,
        _ => unreachable!(),
    }
}

fn eval_binary_sets(lhs: Set, op: Operator, rhs: Set) -> Set {
    match op {
        And => match (lhs.is_complement, rhs.is_complement) {
            (false, false) => lhs
                .values
                .iter()
                .filter(|x| rhs.values.contains(x))
                .copied()
                .collect(),
            (false, true) => lhs
                .values
                .iter()
                .filter(|x| !rhs.values.contains(x))
                .copied()
                .collect(),
            (true, false) => rhs
                .values
                .iter()
                .filter(|x| !lhs.values.contains(x))
                .copied()
                .collect(),
            (true, true) => Set {
                values: [
                    lhs.values.clone(),
                    rhs.values
                        .iter()
                        .filter(|x| !lhs.values.contains(x))
                        .copied()
                        .collect(),
                ]
                .concat(),
                is_complement: true,
            },
        },
        Or => match (lhs.is_complement, rhs.is_complement) {
            (false, false) => [
                lhs.values.clone(),
                rhs.values
                    .iter()
                    .filter(|x| !lhs.values.contains(x))
                    .copied()
                    .collect(),
            ]
            .concat()
            .into(),
            (false, true) => lhs,
            (true, false) => rhs,
            (true, true) => Set {
                values: lhs
                    .values
                    .iter()
                    .filter(|x| rhs.values.contains(x))
                    .copied()
                    .collect(),
                is_complement: true,
            },
        },
        _ => unreachable!(),
    }
}

fn eval_unary(op: Operator, child: bool) -> bool {
    match op {
        Not => !child,
        _ => unreachable!(),
    }
}

fn eval_unary_sets(op: Operator, mut child: Set) -> Set {
    match op {
        Not => {
            child.is_complement = !child.is_complement;
            child
        }
        _ => unreachable!(),
    }
}

impl ops::BitOr for Node {
    type Output = Node;

    fn bitor(self, rhs: Node) -> Node {
        BinaryExpr {
            op: Or,
            lhs: Box::from(self),
            rhs: Box::from(rhs),
        }
    }
}

impl ops::Not for Node {
    type Output = Node;

    fn not(self) -> Node {
        UnaryExpr {
            op: Not,
            child: Box::from(self),
        }
    }
}

impl ops::BitAnd for Node {
    type Output = Node;

    fn bitand(self, rhs: Node) -> Node {
        BinaryExpr {
            op: And,
            lhs: Box::from(self),
            rhs: Box::from(rhs),
        }
    }
}

impl ops::BitXor for Node {
    type Output = Node;

    fn bitxor(self, rhs: Node) -> Node {
        BinaryExpr {
            op: Xor,
            lhs: Box::from(self),
            rhs: Box::from(rhs),
        }
    }
}

#[cfg(test)]
mod node_tests {
    use crate::tree::Tree;
    use std::str::FromStr;

    #[test]
    // lots of rpn tests
    fn test_print_rpn() {
        let teststr = "AB&";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.print_rpn(), teststr);
    }

    #[test]
    fn test_print_rpn_2() {
        let teststr = "AB&C|";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.print_rpn(), teststr);
    }

    #[test]
    fn test_print_rpn_3() {
        let teststr = "AB&C|D^";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.print_rpn(), teststr);
    }

    #[test]
    fn test_print_rpn_4() {
        let teststr = "AB&C|D^!";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.print_rpn(), teststr);
    }
}
