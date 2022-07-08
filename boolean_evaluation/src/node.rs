use crate::operator::Operator;
use crate::variable::Variable;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
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
    pub fn compute_node(self) -> bool {
        match self {
            Variable(v) => v.borrow().value,
            Constant(p) => p,
            BinaryExpr { op, lhs, rhs } => eval_binary(lhs.compute_node(), op, rhs.compute_node()),
            UnaryExpr { op, child } => eval_unary(op, child.compute_node()),
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

fn eval_unary(op: Operator, child: bool) -> bool {
    match op {
        Not => !child,
        _ => unreachable!(),
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
