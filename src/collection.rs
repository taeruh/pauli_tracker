pub trait CollectionRequired {
    type T: Default + Clone;
    type IterMut<'l>: Iterator<Item = (usize, &'l mut Self::T)>
    where
        Self: 'l;

    fn insert(&mut self, key: usize, value: Self::T) -> Option<Self::T>;
    fn remove(&mut self, bit: usize) -> Option<Self::T>;
    fn get_mut(&mut self, bit: usize) -> Option<&mut Self::T>;
    fn get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> Option<(&mut Self::T, &mut Self::T)>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn init(num: usize) -> Self;
}

pub trait Collection:
    CollectionRequired
    + IntoIterator<Item = (usize, Self::T)>
    + FromIterator<(usize, Self::T)>
{
    type Iter<'l>: Iterator<Item = (usize, &'l Self::T)>
    where
        Self: 'l;

    fn iter(&self) -> Self::Iter<'_>;

    fn get(&self, bit: usize) -> Option<&Self::T>;

    fn sort_by_key(&self) -> Vec<(usize, &Self::T)> {
        let mut ret = self.iter().collect::<Vec<(usize, &Self::T)>>();
        ret.sort_by_key(|(i, _)| *i);
        ret
    }

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
mod hash_map;
mod mapped_vector;

pub use buffered_vector::BufferedVector;
pub use hash_map::Map;
pub use mapped_vector::MappedVector;
