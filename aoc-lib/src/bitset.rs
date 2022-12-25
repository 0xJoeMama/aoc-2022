use std::mem;

#[derive(Debug, Clone)]
pub struct BitSet {
    data: Vec<usize>,
    len: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    /// Specify capacity in bits
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap / mem::size_of::<usize>()),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Sets the bit at the given index to true
    pub fn insert(&mut self, index: usize) {
        let bucket = index / mem::size_of::<usize>();
        let bit = index % mem::size_of::<usize>();

        if bucket >= self.data.len() {
            self.data.resize(bucket + 1, 0);
        }

        self.data[bucket] |= 1 << bit;
    }

    pub fn remove(&mut self, index: usize) {
        let bucket = index / mem::size_of::<usize>();
        let bit = index % mem::size_of::<usize>();

        if bucket >= self.data.len() {
            return;
        }

        self.data[bucket] &= !(1 << bit);
    }

    pub fn toggle(&mut self, index: usize) {
        let bucket = index / mem::size_of::<usize>();
        let bit = index % mem::size_of::<usize>();

        if bucket >= self.data.len() {
            self.data.resize(bucket + 1, 0);
        }

        self.data[bucket] ^= 1 << bit;
    }

    pub fn contains(&self, index: usize) -> bool {
        let bucket = index / mem::size_of::<usize>();
        let bit = index % mem::size_of::<usize>();

        if bucket >= self.data.len() {
            return false;
        }

        self.data[bucket] & (1 << bit) != 0
    }

    pub fn get(&self, index: usize) -> bool {
        self.contains(index)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.insert(index);
        } else {
            self.remove(index);
        }
    }

    pub fn iter(&self) -> BitSetIter {
        BitSetIter {
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

        let result = self.bitset.contains(self.idx);
        self.idx += 1;
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bs = BitSet::with_capacity(100);
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
}
