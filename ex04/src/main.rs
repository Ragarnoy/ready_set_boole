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

#[cfg(test)]
mod truth_table_42_test {
    use boolean_evaluation::tree::Tree;
    use boolean_evaluation::truth_table::TruthTable;


    #[test]
    fn test_truth_table_42() {
        let example =
            "| A | B | C | = |\n\
            |---|---|---|---|\n\
            | 0 | 0 | 0 | 0 |\n\
            | 1 | 0 | 0 | 0 |\n\
            | 0 | 1 | 0 | 0 |\n\
            | 1 | 1 | 0 | 1 |\n\
            | 0 | 0 | 1 | 1 |\n\
            | 1 | 0 | 1 | 1 |\n\
            | 0 | 1 | 1 | 1 |\n\
            | 1 | 1 | 1 | 1 |\n";
        let output = TruthTable::from("AB&C|".parse::<Tree>().unwrap());
        assert_eq!(example, output.to_string());
    }
}