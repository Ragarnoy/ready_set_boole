#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: char,
    pub value: bool,
    pub set: Option<Vec<i32>>,
}

impl Variable {
    pub fn new(name: char) -> Self {
        Self {
            name,
            value: false,
            set: None,
        }
    }
}
