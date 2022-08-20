use crate::tree::Tree;
use std::fmt::{Display, Formatter};

pub struct TruthTable {
    variables: Vec<char>,
    values: Vec<Vec<bool>>,
}

impl Display for TruthTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut header = String::with_capacity(self.variables.len() * 3 + 4);
        self.variables.iter().for_each(|var| {
            header.push_str(format!("| {} ", var).as_str());
        });
        header.push_str("| = |");
        let separator = header.replace(|ch| ch != '|', "-");
        let mut body = String::with_capacity(self.variables.len() * self.variables.len() * 3 + 3);
        self.values.iter().for_each(|row| {
            for val in row.iter() {
                body.push_str(format!("| {} ", *val as u8).as_str());
            }
            body.push_str("|\n");
        });
        write!(f, "{}\n{}\n{}", header, separator, body)
    }
}

impl From<Tree> for TruthTable {
    fn from(tree: Tree) -> Self {
        if tree.variable_list.is_none() {
            panic!("Tree must have a variable list");
        }
        if let Some(variable_list) = tree.variable_list.as_ref() {
            let values: Vec<Vec<bool>> = (0..2u32.pow(variable_list.len() as u32))
                .map(|bitfield| {
                    let mut tmp = variable_list
                        .iter()
                        .enumerate()
                        .map(|(i, v)| {
                            let mask = (bitfield & (1u32 << i)) != 0;
                            if let Some(v) = v {
                                v.borrow_mut().value = mask;
                            }
                            mask
                        })
                        .collect::<Vec<bool>>();
                    tmp.push(tree.clone().root.eval_ref());
                    tmp
                })
                .collect();
            Self {
                variables: variable_list
                    .iter()
                    .map(|v| v.as_ref().unwrap().borrow().name)
                    .collect(),
                values,
            }
        } else {
            panic!("Tree must have a variable list")
        }
    }
}
