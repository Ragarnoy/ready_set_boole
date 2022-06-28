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

                match has_and {
                    (true, true) => distribute_both(Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    }),
                    (true, false) => distribute_to_left(Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    }),
                    (false, true) => distribute_to_right(Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    }),
                    _ => Node::BinaryExpr {
                        op: Operator::Or,
                        lhs: Box::new(node_to_cnf(*lhs)),
                        rhs: Box::new(node_to_cnf(*rhs)),
                    },
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
    if let Node::BinaryExpr {
        op: Operator::Or,
        lhs,
        rhs,
    } = node
    {
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
            ) => Node::BinaryExpr {
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
            },
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn distribute_to_left(node: Node) -> Node {
    if let Node::BinaryExpr {
        op: Operator::Or,
        lhs,
        rhs,
    } = node
    {
        match *lhs {
            Node::BinaryExpr {
                op: Operator::And,
                lhs: lhs_lhs,
                rhs: lhs_rhs,
            } => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(node_to_cnf(Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(*lhs_lhs),
                    rhs: Box::new(*rhs.clone()),
                })),
                rhs: Box::new(node_to_cnf(Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(*lhs_rhs),
                    rhs: Box::new(*rhs),
                })),
            },
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn distribute_to_right(node: Node) -> Node {
    if let Node::BinaryExpr {
        op: Operator::Or,
        lhs,
        rhs,
    } = node
    {
        match *rhs {
            Node::BinaryExpr {
                op: Operator::And,
                lhs: rhs_lhs,
                rhs: rhs_rhs,
            } => Node::BinaryExpr {
                op: Operator::And,
                lhs: Box::new(node_to_cnf(Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(*lhs.clone()),
                    rhs: Box::new(*rhs_lhs),
                })),
                rhs: Box::new(node_to_cnf(Node::BinaryExpr {
                    op: Operator::Or,
                    lhs: Box::new(*lhs),
                    rhs: Box::new(*rhs_rhs),
                })),
            },
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod cnf_test {
    use crate::cnf::{distribute_both, distribute_to_left, distribute_to_right, node_to_cnf};
    use crate::node::Node;
    use std::str::FromStr;

    #[test]
    fn basic_node_to_cnf() {
        let node = Node::from_str("").unwrap();
    }

    #[test]
    fn basic_distribute_both() {
        let node = Node::from_str("AB&CD&|").unwrap();
        let result = Node::from_str("AC|AD|&BC|BD|&&").unwrap();
        assert_eq!(distribute_both(node), result);
    }

    #[test]
    fn basic_distribute_left() {
        let node = Node::from_str("AB&C|").unwrap();
        let result = Node::from_str("AC|BC|&").unwrap();
        assert_eq!(distribute_to_left(node), result);
    }

    #[test]
    fn basic_distribute_right() {
        let node = Node::from_str("CAB&|").unwrap();
        let result = Node::from_str("CA|CB|&").unwrap();
        assert_eq!(distribute_to_right(node), result);
    }
}
