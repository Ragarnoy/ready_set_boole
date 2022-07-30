use boolean_evaluation::cnf::node_to_cnf;
use boolean_evaluation::tree::Tree;
use std::str::FromStr;

fn conjunctive_normal_form(formula: &str) -> String {
    let node = Tree::from_str(formula).unwrap();
    node_to_cnf(node.root).print_rpn()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        conjunctive_normal_form(&args[1]);
    } else {
        conjunctive_normal_form("AB&C|");
    }
}

#[cfg(test)]
mod cnf_42_test {
    use crate::conjunctive_normal_form;

    #[test]
    fn test_cnf_42() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");

        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");

        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");

        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");

        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");

        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");

        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }
}
