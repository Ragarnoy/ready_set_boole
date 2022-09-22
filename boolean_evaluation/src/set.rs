use std::ops::Not;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Set {
    pub values: Vec<i32>,
    pub is_complement: bool,
}

impl Set {
    pub fn complement(self) -> Self {
        Self {
            values: self.values,
            is_complement: !self.is_complement,
        }
    }

    pub fn union(self, other: Self) -> Self {
        [
            self.values.clone(),
            other.values
                .iter()
                .filter(|x| !self.values.contains(x))
                .copied()
                .collect(),
        ]
            .concat()
            .into()
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

impl Not for Set {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            values: self.values,
            is_complement: !self.is_complement,
        }
    }
}

impl std::ops::BitOr for Set {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.is_complement, rhs.is_complement) {
            (false, false) => self.union(rhs),
            (true, true) => Set {
                values: self
                    .values
                    .iter()
                    .filter(|x| rhs.values.contains(x))
                    .copied()
                    .collect(),
                is_complement: true,
            },
            (true, false) => Self {
                values: self
                    .values
                    .into_iter()
                    .filter(|x| !rhs.values.contains(x))
                    .collect(),
                is_complement: true,
            },
            (false, true) => Self {
                values: rhs
                    .values
                    .into_iter()
                    .filter(|x| !self.values.contains(x))
                    .collect(),
                is_complement: true,
            },
        }
    }
}

impl std::ops::BitAnd for Set {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.is_complement, rhs.is_complement) {
            (false, false) => Self {
                values: self
                    .values
                    .into_iter()
                    .filter(|x| rhs.values.contains(x))
                    .collect(),
                is_complement: false,
            },
            (true, true) => Self {
                values: [
                    self.values.clone(),
                    rhs.values
                        .iter()
                        .filter(|x| !self.values.contains(x))
                        .copied()
                        .collect(),
                ]
                .concat(),
                is_complement: true,
            },
            (true, false) => Self {
                values: self
                    .values
                    .into_iter()
                    .filter(|x| !rhs.values.contains(x))
                    .collect(),
                is_complement: true,
            },
            (false, true) => Self {
                values: rhs
                    .values
                    .into_iter()
                    .filter(|x| !self.values.contains(x))
                    .collect(),
                is_complement: true,
            },
        }
    }
}

