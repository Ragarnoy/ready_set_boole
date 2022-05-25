use std::str::FromStr;
use boolean_evaluation::node::Node;

fn eval_formula(str: &str) -> bool {
    let node = Node::from_str(str).unwrap();
    node.compute_node()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        eval_formula(&args[1]);
    } else {
        eval_formula("10=");
    }
}

#[cfg(test)]
mod test_bool_eval {
    use super::*;
    use std::panic;

    #[test]
    fn test_inputs() {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        let invalid_unary = panic::catch_unwind(|| eval_formula("!1"));
        let not_enough_tokens = panic::catch_unwind(|| eval_formula("!"));
        let invalid_token = panic::catch_unwind(|| eval_formula("@"));
        let no_input = panic::catch_unwind(|| eval_formula(""));
        assert!(invalid_unary.is_err());
        assert!(not_enough_tokens.is_err());
        assert!(invalid_token.is_err());
        assert!(no_input.is_err());
    }

    #[test]
    fn test_ands() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("11&"));
        assert!(!eval_formula("00&"));
    }

    #[test]
    fn test_ors() {
        assert!(eval_formula("10|"));
        assert!(eval_formula("11|"));
        assert!(!eval_formula("00|"));
    }

    #[test]
    fn test_xors() {
        assert!(eval_formula("10^"));
        assert!(!eval_formula("11^"));
        assert!(!eval_formula("00^"));
    }

    #[test]
    fn test_negs() {
        assert!(eval_formula("0!"));
        assert!(!eval_formula("1!"));
    }

    #[test]
    fn test_imply() {
        assert!(eval_formula("11>"));
        assert!(eval_formula("00>"));
        assert!(!eval_formula("01>"));
        assert!(eval_formula("10>"));
    }

    #[test]
    fn test_leqs() {
        assert!(eval_formula("11="));
        assert!(!eval_formula("01="));
        assert!(eval_formula("00="));
        assert!(!eval_formula("10="));
    }

    #[test]
    fn test_examples() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }
}
