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
            BinaryExpr { op, lhs, rhs } => op.eval_binary(lhs.eval(), rhs.eval()),
            UnaryExpr { op, child } => op.eval_unary(child.eval()),
        }
    }

    pub fn eval_ref(&self) -> bool {
        match self {
            Variable(v) => v.borrow().value,
            Constant(p) => *p,
            BinaryExpr { op, lhs, rhs } => op.eval_binary(lhs.eval_ref(), rhs.eval_ref()),
            UnaryExpr { op, child } => op.eval_unary(child.eval_ref()),
        }
    }

    pub fn compute_sets(&self) -> Set {
        match self {
            Variable(v) => Set::from(v.borrow().set.as_ref().unwrap().clone()),
            Constant(p) => Set::from(vec![if *p { 1 } else { 0 }]),
            BinaryExpr { op, lhs, rhs } => {
                let lhs_sets = lhs.compute_sets();
                let rhs_sets = rhs.compute_sets();
                op.eval_binary_sets(lhs_sets, rhs_sets)
            }
            UnaryExpr { op, child } => {
                let child_sets = child.compute_sets();
                op.eval_unary_sets(child_sets)
            }
        }
    }

    pub fn to_rpn(&self) -> String {
        let mut ret = String::new();
        match self {
            // for a variable or constant, just print the value
            Variable(c) => ret.push(RefCell::borrow(c).name),
            Constant(x) => ret.push(if *x { '1' } else { '0' }),
            // for a unary expression, first recurse on the child, then print the operator
            UnaryExpr { op, child } => ret.push_str(&*format!("{}{:?}", Self::to_rpn(child), op)),
            // for a binary expression, first recurse on the lhs, then recurse on the rhs, then print the operator
            BinaryExpr { op, lhs, rhs } => ret.push_str(&*format!(
                "{}{}{:?}",
                Self::to_rpn(lhs),
                Self::to_rpn(rhs),
                op
            )),
        }
        ret
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
    fn test_to_rpn() {
        let teststr = "AB&";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.to_rpn(), teststr);
    }

    #[test]
    fn test_to_rpn_2() {
        let teststr = "AB&C|";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.to_rpn(), teststr);
    }

    #[test]
    fn test_to_rpn_3() {
        let teststr = "AB&C|D^";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.to_rpn(), teststr);
    }

    #[test]
    fn test_to_rpn_4() {
        let teststr = "AB&C|D^!";
        let node = Tree::from_str(teststr).unwrap();
        assert_eq!(node.root.to_rpn(), teststr);
    }
}
