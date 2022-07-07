use std::cell::RefCell;
use crate::operator::Operator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
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
            Node::Variable(c) => ret.push(RefCell::borrow(c).name),
            Node::Constant(x) => ret.push(if *x { '1' } else { '0' }),
            Node::UnaryExpr { op, child } => ret.push_str(&*format!("{}{}", op, child)),
            Node::BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!("{} {} {}", lhs, op, rhs)),
        }
        write!(f, "{}", ret)
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
            Node::Variable(c) => ret.push(RefCell::borrow(c).name),
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
    use crate::tree::Tree;

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
