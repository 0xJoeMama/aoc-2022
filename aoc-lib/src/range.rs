use std::{
    convert::Infallible,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<T>
where
    T: Copy + Add + Ord + PartialOrd + Eq + PartialEq + Mul,
{
    min: T,
    max: T,
}

pub enum Relative {
    Subrange,
    Overlaps,
    Disjoint,
}

impl<T> Range<T>
where
    T: Copy + Add + Sub<Output = T> + Ord + PartialOrd + Eq + PartialEq + Mul,
{
    pub fn new(min: T, max: T) -> Range<T> {
        Range { min, max }
    }

    pub fn contains(&self, i: T) -> bool {
        i <= self.max && i >= self.min
    }

    /// Checks if other is a subrange of self.
    pub fn subrange(&self, other: &Range<T>) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    /// Checks if other overlaps with self.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }

    /// Gives the relative position of other to self.
    pub fn relative(&self, other: &Self) -> Relative {
        if self.subrange(other) {
            Relative::Subrange
        } else if self.overlaps(other) {
            Relative::Overlaps
        } else {
            Relative::Disjoint
        }
    }

    pub fn get_max(&self) -> T {
        self.max
    }

    pub fn mut_max(&mut self) -> &mut T {
        &mut self.max
    }

    pub fn get_min(&self) -> T {
        self.min
    }

    pub fn mut_min(&mut self) -> &mut T {
        &mut self.min
    }

    pub fn get_span(&self) -> T {
        self.max - self.min
    }
}

impl<A> FromIterator<Range<A>> for (Range<A>, Range<A>)
where
    A: Copy + Add + Ord + PartialOrd + Eq + PartialEq + Mul,
{
    fn from_iter<T: IntoIterator<Item = Range<A>>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let first = iter.next().unwrap();
        let last = iter.next().unwrap();
        (first, last)
    }
}

impl FromStr for Range<i32> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').unwrap();
        Ok(Self::new(
            min.parse::<i32>().unwrap(),
            max.parse::<i32>().unwrap(),
        ))
    }
}
