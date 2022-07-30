use boolean_evaluation::nnf;

use boolean_evaluation::tree::Tree;
use std::str::FromStr;

fn negation_normal_form(formula: &str) -> String {
    let node = Tree::from_str(formula).unwrap();
    nnf::node_to_negation_normal_form(node.root).print_rpn()
}

fn main() {
    println!("{}", negation_normal_form("AB&!"));
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
