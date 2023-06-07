use super::BoolVector;

#[derive(Clone, PartialEq, Debug)]
pub struct BitVec(pub bitvec_simd::BitVec);

impl FromIterator<bool> for BitVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut res = BitVec::zeros(iter.size_hint().1.unwrap());
        for (i, f) in iter.enumerate() {
            res.0.set(i, f);
        }
        res
    }
}

pub struct Iter {
    vec: BitVec,
    current: usize,
}
impl Iterator for Iter {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.vec.0.get(self.current - 1)
    }
}

pub struct IterFromRef<'l> {
    vec: &'l BitVec,
    current: usize,
}
impl<'l> Iterator for IterFromRef<'l> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.vec.0.get(self.current - 1)
    }
}

impl IntoIterator for BitVec {
    type Item = bool;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter { vec: self, current: 0 }
    }
}

impl BoolVector for BitVec {
    type Iter<'l> = IterFromRef<'l>;

    fn new() -> Self {
        Self::zeros(0)
    }

    fn zeros(len: usize) -> Self {
        Self(bitvec_simd::BitVec::zeros(len))
    }

    fn set(&mut self, idx: usize, flag: bool) {
        self.0.set(idx, flag);
    }

    fn xor_inplace(&mut self, rhs: &Self) {
        self.0.xor_inplace(&rhs.0);
    }

    fn or_inplace(&mut self, rhs: &Self) {
        self.0.or_inplace(&rhs.0);
    }

    fn resize(&mut self, len: usize, flag: bool) {
        self.0.resize(len, flag);
    }

    fn push(&mut self, flag: bool) {
        let len = self.bits();
        if len == 0 {
            if flag {
                self.0 = bitvec_simd::BitVec::ones(1);
            } else {
                self.0 = bitvec_simd::BitVec::zeros(1);
            }
        } else {
            self.0.set(len, flag);
        }
    }

    fn pop(&mut self) -> Option<bool> {
        let last = self.bits().checked_sub(1)?;
        // last > self.0.len is not possible because of the above
        let res = self.0.get_unchecked(last);
        self.0.shrink_to(last);
        Some(res)
    }

    fn bits(&self) -> usize {
        self.0.len()
    }

    fn iter_vals(&self) -> Self::Iter<'_> {
        IterFromRef { vec: self, current: 0 }
    }
}
