use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberRange {
    min: i32,
    max: i32,
}

impl NumberRange {
    pub fn new(min: i32, max: i32) -> NumberRange {
        NumberRange { min, max }
    }

    pub fn subrange(&self, other: &NumberRange) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    pub fn contains(&self, i: i32) -> bool {
        i <= self.max && i >= self.min
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }
}

impl FromIterator<NumberRange> for (NumberRange, NumberRange) {
    fn from_iter<T: IntoIterator<Item = NumberRange>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let first = iter.next().unwrap();
        let last = iter.next().unwrap();
        (first, last)
    }
}

impl FromStr for NumberRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split('-').map(|inner| inner.parse().unwrap());
        Ok(NumberRange::new(
            pairs.next().unwrap(),
            pairs.next().unwrap(),
        ))
    }
}
