use boolean_evaluation::tree::Tree;
use std::str::FromStr;

fn sat(formula: &str) -> bool {
    let node = Tree::from_str(formula).unwrap();
    node.sat()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        println!("{}", sat(&args[1]));
    } else {
        println!("{}", sat("AA^"));
    }
}

#[cfg(test)]
mod sat_42_test {
    use crate::sat;

    #[test]
    fn test_sat_42() {
        // TRUE
        assert!(sat("AB|"));
        assert!(sat("AB&"));

        // FALSE
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }
}