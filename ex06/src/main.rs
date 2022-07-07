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
