use boolean_evaluation::cnf::node_to_cnf;
use boolean_evaluation::tree::Tree;
use std::str::FromStr;

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let mut node = Tree::from_str(formula).unwrap();
    node.assign_sets(sets);
    node.root = node_to_cnf(node.root);
    node.evaluate_sets()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        let formula = &args[1];
        let sets: Vec<Vec<i32>> = args[2..]
            .iter()
            .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();
        println!("{:?}", eval_set(formula, sets));
    } else {
        println!("{:?}", eval_set("AB&", vec![vec![0, 1, 2], vec![0, 1, 3]]));
    }
}

#[cfg(test)]
mod set_evaluation_42_tests {
    use crate::eval_set;

    #[test]
    fn test_set_evaluation_42() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        assert_eq!(eval_set("AB&", sets), vec![0]);
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        assert_eq!(eval_set("AB|", sets), vec![0, 1, 2, 3, 4, 5]);
        let sets = vec![vec![0, 1, 2]];
        assert_eq!(eval_set("A!", sets), vec![]);
    }
}