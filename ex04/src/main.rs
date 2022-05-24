mod node;
mod operator;
mod truth_table;

use crate::node::Node;
use crate::truth_table::TruthTable;
use std::str::FromStr;

fn print_truth_table(formula: &str) {
    let truth_table = TruthTable::from_str(formula).unwrap();
    print!("{}", truth_table);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        print_truth_table(&args[1]);
    } else {
        print_truth_table("AB&C|");
    }
}

#[cfg(test)]
mod test_truth_table {}
