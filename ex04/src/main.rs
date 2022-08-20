use boolean_evaluation::tree::Tree;
use boolean_evaluation::truth_table::TruthTable;

fn print_truth_table(formula: &str) {
    let truth_table = TruthTable::from(formula.parse::<Tree>().unwrap());
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
