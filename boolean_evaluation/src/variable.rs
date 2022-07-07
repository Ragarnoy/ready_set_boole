#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: char,
    pub value: bool,
}

impl Variable {
    pub fn new(name: char, value: bool) -> Self {
        Self { name, value }
    }
}
