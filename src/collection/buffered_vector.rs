use core::slice;
use std::{
    cmp::Ordering,
    iter::{
        self,
        Enumerate,
    },
    mem,
    ops::{
        Deref,
        DerefMut,
    },
};

use super::{
    CollectionRequired,
    Collection,
};
use crate::slice_extension::GetTwoMutSlice;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct BufferedVector<T>(pub Vec<T>);

impl<T> BufferedVector<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Deref for BufferedVector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for BufferedVector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for BufferedVector<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> FromIterator<(usize, T)> for BufferedVector<T> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut res = Vec::new();
        for (key, value) in iter {
            res.insert(key, value);
        }
        Self(res)
    }
}

impl<'l, T> IntoIterator for &'l BufferedVector<T> {
    type Item = (usize, &'l T);
    type IntoIter = Enumerate<slice::Iter<'l, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().enumerate()
    }
}

impl<'l, T> IntoIterator for &'l mut BufferedVector<T> {
    type Item = (usize, &'l mut T);
    type IntoIter = Enumerate<slice::IterMut<'l, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut().enumerate()
    }
}

impl<T> IntoIterator for BufferedVector<T> {
    type Item = (usize, T);
    type IntoIter = Enumerate<<Vec<T> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().enumerate()
    }
}

impl<T: Default + Clone> CollectionRequired for BufferedVector<T> {
    type T = T;
    type IterMut<'l> = <&'l mut Self as IntoIterator>::IntoIter where T: 'l;

    fn insert(&mut self, key: usize, value: T) -> Option<T> {
        let len = self.len();
        match key.cmp(&len) {
            Ordering::Less => Some(mem::replace(
                self.get_mut(key)
                    .expect("can't be out of bounds in this match arm"),
                value,
            )),
            Ordering::Equal => {
                self.push(value);
                None
            }
            Ordering::Greater => {
                let diff = key - len;
                self.try_reserve(diff).unwrap_or_else(|e| {
                    panic!("error when trying to reserve enough memory: {e}")
                });
                self.extend(iter::repeat(T::default()).take(diff));
                self.push(value);
                None
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        match key.cmp(&(self.len().checked_sub(1)?)) {
            Ordering::Less => panic!(
                "this type, which is basically a Vec, only allows removing elements \
                 consecutively from the end"
            ),
            Ordering::Equal => Some(
                self.pop()
                    .expect("bug: we checked above that len is bigger than 0"),
            ),
            Ordering::Greater => None,
        }
    }

    #[inline(always)]
    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.0.get_mut(key)
    }

    fn get_two_mut(&mut self, key_a: usize, key_b: usize) -> Option<(&mut T, &mut T)> {
        self.0.get_two_mut(key_a, key_b)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline(always)]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }

    #[inline(always)]
    fn init(num_keys: usize) -> Self {
        Self(vec![T::default(); num_keys])
    }
}

impl<T: Default + Clone> Collection for BufferedVector<T> {
    type Iter<'l> = <&'l Self as IntoIterator>::IntoIter where T: 'l;

    #[inline(always)]
    fn get(&self, key: usize) -> Option<&T> {
        self.0.get(key)
    }

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}
