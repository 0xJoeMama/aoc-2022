use std::{fmt::Display, mem};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BitSet {
    data: Vec<usize>,
    len: usize,
}

impl BitSet {
    const BUCKET_SIZE: usize = mem::size_of::<usize>() * 8; // in bits

    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap / Self::BUCKET_SIZE),
            len: 0,
        }
    }

    #[must_use]
    pub fn all_false(cap: usize) -> Self {
        Self {
            data: vec![0; cap / Self::BUCKET_SIZE + 1],
            len: cap,
        }
    }

    #[must_use]
    pub fn all_true(cap: usize) -> Self {
        Self {
            data: vec![usize::MAX; cap / Self::BUCKET_SIZE],
            len: cap,
        }
    }

    pub fn insert(&mut self, value: bool) {
        if self.len % Self::BUCKET_SIZE == 0 {
            self.data.push(0);
        }

        if value {
            self.data[self.len / Self::BUCKET_SIZE] |= 1 << (self.len % Self::BUCKET_SIZE);
        }

        self.len += 1;
    }

    pub fn toggle(&mut self, index: usize) {
        self.data[index / Self::BUCKET_SIZE] ^= 1 << (index % Self::BUCKET_SIZE);
    }

    #[must_use]
    pub fn get(&self, index: usize) -> bool {
        self.data[index / Self::BUCKET_SIZE] & (1 << (index % Self::BUCKET_SIZE)) != 0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn iter(&self) -> BitSetIter {
        BitSetIter {
            bitset: self,
            idx: 0,
        }
    }

    #[must_use]
    pub fn iter_true(&self) -> BitSetTrueIter {
        BitSetTrueIter {
            bitset: self,
            idx: 0,
        }
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BitSetIter<'a> {
    bitset: &'a BitSet,
    idx: usize,
}

impl<'a> Iterator for BitSetIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.bitset.len {
            return None;
        }

        let result = self.bitset.get(self.idx);
        self.idx += 1;
        Some(result)
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            if self.get(i) {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
        }

        Ok(())
    }
}

pub struct BitSetTrueIter<'a> {
    bitset: &'a BitSet,
    idx: usize,
}

impl<'a> Iterator for BitSetTrueIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.bitset.len {
            if self.bitset.get(self.idx) {
                let result = self.idx;
                self.idx += 1;
                return Some(result);
            }

            self.idx += 1;
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bs = BitSet::all_false(10);
        assert!(bs.iter().all(|b| !b));
        for i in 0..100 {
            assert!(!bs.get(i));
            bs.toggle(i);
        }

        for i in 0..100 {
            assert!(bs.get(i));
            bs.toggle(i);
        }

        assert!(bs.iter().all(|b| b));
    }

    #[test]
    fn test_bitset_true() {
        let mut bs = BitSet::all_false(10);
        assert!(bs.iter_true().next().is_none());
        for i in 0..100 {
            assert!(!bs.get(i));
            bs.toggle(i);
        }

        for i in 0..100 {
            assert!(bs.get(i));
            bs.toggle(i);
        }

        assert!(bs.iter_true().next().is_none());
    }
}
