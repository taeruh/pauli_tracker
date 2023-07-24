pub trait Base {
    type TB;
    fn insert(&mut self, key: usize, value: Self::TB) -> Option<Self::TB>;
    fn remove(&mut self, bit: usize) -> Option<Self::TB>;
    fn get(&self, bit: usize) -> Option<&Self::TB>;
    fn get_mut(&mut self, bit: usize) -> Option<&mut Self::TB>;
    fn get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> Option<(&mut Self::TB, &mut Self::TB)>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Iterable {
    type TI;
    type Iter<'l>: Iterator<Item = (usize, &'l Self::TI)>
    where
        Self: 'l;
    type IterMut<'l>: Iterator<Item = (usize, &'l mut Self::TI)>
    where
        Self: 'l;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn sort_by_key(&self) -> Vec<(usize, &Self::TI)> {
        let mut ret = self.iter().collect::<Vec<(usize, &Self::TI)>>();
        ret.sort_by_key(|(i, _)| *i);
        ret
    }
}

pub trait Init {
    fn init(len: usize) -> Self;
}

pub trait IterableBase: Base<TB = Self::T> + Iterable<TI = Self::T> {
    type T;
}

pub trait Full:
    IterableBase
    + Init
    + IntoIterator<Item = (usize, Self::TI)>
    + FromIterator<(usize, Self::TI)>
{
    fn into_sorted_by_key(self) -> Vec<(usize, Self::T)>
    where
        Self: Sized,
    {
        let mut ret = self.into_iter().collect::<Vec<(usize, Self::T)>>();
        ret.sort_by_key(|(i, _)| *i);
        ret
    }
}

mod buffered_vector;
mod map;
mod mapped_vector;

pub use buffered_vector::BufferedVector;
pub use map::Map;
pub use mapped_vector::MappedVector;
