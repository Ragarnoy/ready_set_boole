use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Node;

const VALID_TOKENS: &[char] = &['1', '0', '!', '&', '^', '=', '|', '>'];

pub struct TruthTable {
    variables: Vec<String>,
    initial_formula: String,
}

impl FromStr for TruthTable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(String::from("Empty input!"))
        }
        if !s.contains(VALID_TOKENS) || !s.contains(|ch| ('A'..='Z').contains(&ch)) {
            return Err(String::from("Invalid tokens"));
        }


        let mut variables: Vec<String> = s.chars().filter(|ch| ch.is_ascii_uppercase()).map(|ch| ch.to_string()).collect();
        variables.sort();
        variables.dedup();

        Ok(TruthTable {
            variables,
            initial_formula: s.to_string(),
        })
    }

}

impl Display for TruthTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut header = String::with_capacity(self.variables.len() * 3 + 3);
        self.variables.iter().for_each(|var| {
            header.push_str(format!("| {} ", var).as_str());
        });
        header.push_str("| = |");

        let separator = header.replace(|ch| ch != '|', "-");

        let mut body = String::with_capacity(self.variables.len() * self.variables.len() * 3 + 3);

        for bitfield in 0..2_u32.pow(self.variables.len() as u32) {
            let mut tmp_formula = self.initial_formula.clone();
            let mut tmp_bodyline = header.clone();
            for (i, var) in self.variables.iter().rev().enumerate() {
                tmp_formula = tmp_formula.replace(
                    var,
                    if bitfield & (1 << i as u32) == 0 {
                        "0"
                    } else {
                        "1"
                    },
                );
                tmp_bodyline = tmp_bodyline.replace(
                    var,
                    if bitfield & (1 << i as u32) == 0 {
                        "0"
                    } else {
                        "1"
                    },
                );
            }
            let result = Node::from_str(&*tmp_formula).unwrap().compute_node();
            body.push_str(format!("{}\n", tmp_bodyline.replace('=', if result { "1" } else { "0" })).as_str());
        }

        write!(f, "{}\n{}\n{}", header, separator, body)
    }
}