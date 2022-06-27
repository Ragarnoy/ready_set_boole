use crate::nnf::node_to_negation_normal_form;
use crate::node::Node;
use crate::operator::Operator;

pub fn node_to_cnf(node: Node) -> Node {
    let node = node_to_negation_normal_form(node);
    if let Node::BinaryExpr { op, lhs, rhs } = node {
        match op {
            Operator::And => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(node_to_cnf(*lhs)),
                rhs: Box::new(node_to_cnf(*rhs)),
            },
            Operator::Or => {
                let has_and = (
                    matches!(
                        *lhs,
                        Node::BinaryExpr {
                            op: Operator::And,
                            ..
                        }
                    ),
                    matches!(
                        *rhs,
                        Node::BinaryExpr {
                            op: Operator::And,
                            ..
                        }
                    ),
                );
                if has_and == (true, true) {
                    distribute_both(Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    })
                } else if has_and == (true, false) {
                    distribute_left(Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    })
                } else if has_and == (false, true) {
                    distribute_right(Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    })
                } else {
                    Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    }
                }
            }
            _ => unreachable!(),
        }
    } else {
        node
    }
}

// Distribute conjunctions over disjunctions.
fn distribute_both(node: Node) -> Node {
    if let Node::BinaryExpr { op: Operator::Or, lhs, rhs } = node {
        match (*lhs, *rhs) {
            (
                Node::BinaryExpr {
                    op: Operator::And,
                    lhs: lhs_lhs,
                    rhs: lhs_rhs,
                },
                Node::BinaryExpr {
                    op: Operator::And,
                    lhs: rhs_lhs,
                    rhs: rhs_rhs,
                },
            ) => {
                return Node::BinaryExpr {
                    op: Operator::And,
                    lhs: Box::new(Node::BinaryExpr {
                        op: Operator::And,
                        lhs: Box::new(node_to_cnf(Node::BinaryExpr {
                            op: Operator::Or,
                            lhs: Box::new(*lhs_lhs.clone()),
                            rhs: Box::new(*rhs_lhs.clone()),
                        })),
                        rhs: Box::new(node_to_cnf(Node::BinaryExpr {
                            op: Operator::Or,
                            lhs: Box::new(*lhs_lhs),
                            rhs: Box::new(*rhs_rhs.clone()),
                        })),
                    }),
                    rhs: Box::new(Node::BinaryExpr {
                        op: Operator::And,
                        lhs: Box::new(node_to_cnf(Node::BinaryExpr {
                            op: Operator::Or,
                            lhs: Box::new(*lhs_rhs.clone()),
                            rhs: Box::new(*rhs_lhs),
                        })),
                        rhs: Box::new(node_to_cnf(Node::BinaryExpr {
                            op: Operator::Or,
                            lhs: Box::new(*lhs_rhs),
                            rhs: Box::new(*rhs_rhs),
                        })),
                    }),
                }
            },
            _ => unreachable!(),
        }
        }
        todo!()
    }

fn distribute_left(node: Node) -> Node {
    todo!()
}

fn distribute_right(node: Node) -> Node {
    todo!()
}

#[cfg(test)]
mod cnf_test {
    use std::str::FromStr;
    use crate::cnf::node_to_cnf;
    use crate::node::Node;

    #[test]
    fn distribute_both() {
        let node = Node::from_str("AB&CD&|").unwrap();
        let result = Node::from_str("AC|AD|&BC|BD|&&").unwrap();
        println!("{}", node_to_cnf(node.clone()).print_rpn());
        assert_eq!(node_to_cnf(node), result);
    }
}