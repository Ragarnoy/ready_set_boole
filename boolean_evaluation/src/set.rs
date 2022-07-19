pub struct Set {
    pub values: Vec<i32>,
    pub is_complement: bool,
}

impl Set {
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            values,
            is_complement: false,
        }
    }
}

impl From<Vec<i32>> for Set {
    fn from(values: Vec<i32>) -> Self {
        Self::new(values)
    }
}

impl FromIterator<i32> for Set {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}
