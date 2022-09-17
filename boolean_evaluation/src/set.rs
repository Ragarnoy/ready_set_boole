use std::ops::Not;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Set {
    pub values: Vec<i32>,
    pub is_complement: bool,
}

impl Not for Set {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            values: self.values,
            is_complement: !self.is_complement,
        }
    }
}

impl Set {
    pub fn complement(self) -> Self {
        Self {
            values: self.values,
            is_complement: !self.is_complement,
        }
    }
}

impl From<Vec<i32>> for Set {
    fn from(values: Vec<i32>) -> Self {
        Self {
            values,
            is_complement: false,
        }
    }
}

impl FromIterator<i32> for Set {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            values: iter.into_iter().collect(),
            is_complement: false,
        }
    }
}
