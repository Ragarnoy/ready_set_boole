use boolean_evaluation::nnf;
use boolean_evaluation::node::Node;
use std::str::FromStr;
use boolean_evaluation::tree::Tree;

fn negation_normal_form(formula: &str) -> String {
    let node = Tree::from_str(formula).unwrap();
    nnf::node_to_negation_normal_form(node.root).print_rpn()
}

fn main() {
    println!("{}", negation_normal_form("AB&!"));
}
