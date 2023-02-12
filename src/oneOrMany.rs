#![allow(dead_code)]

enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T: Clone> OneOrMany<T> {
    pub fn is_one(&self) -> bool {
        match self {
            Self::One(_) => true,
            Self::Many(_) => false,
        }
    }
    pub fn push(&mut self, new: T) -> usize {
        match self {
            Self::One(a) => {
                *self = Self::Many(vec![a.clone(), new]);
                2
            }
            Self::Many(v) => {
                v.push(new);
                v.len()
            }
        }
    }
}
