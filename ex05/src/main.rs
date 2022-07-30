use boolean_evaluation::nnf;

use boolean_evaluation::tree::Tree;
use std::str::FromStr;

fn negation_normal_form(formula: &str) -> String {
    let node = Tree::from_str(formula).unwrap();
    nnf::node_to_nnf(node.root).to_rpn()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        println!("{}", negation_normal_form(&args[1]));
    } else {
        println!("{}", negation_normal_form("AB&C|"));
    }
}

#[cfg(test)]
mod nnf_42_test {
    use crate::negation_normal_form;

    #[test]
    fn test_nnf_42() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");

        assert_eq!(negation_normal_form("AB|!"), "A!B!&");

        assert_eq!(negation_normal_form("AB>"), "A!B|");

        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");

        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }
}
