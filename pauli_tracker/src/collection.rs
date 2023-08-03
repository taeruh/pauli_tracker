/*!
Different traits to describe behavior of collections.

Semantically, the API is oriented at the [HashMap](std::collections::HashMap) API,
however, the traits are not meant to be a general good API for collections, but rather
for our use cases.
*/

/// A very basic interface for a collection of elements.
///
/// It is defined to be the minimal[^note] interface that we need for the
/// [Live](crate::tracker::live::Live) tracker.
///
/// [^note]: The [get](Base::get), [len](Base::len) and [is_empty](Base::is_empty)
/// methods are actually not needed, but they just make sense here.
pub trait Base {
    /// The type of the elements in the collection
    ///
    /// This type should be equal to [Iterable::TI] and [IterableBase::T], if they are
    /// implemented.
    type TB;

    /// Insert an element into the collection. Returns the previous element at the given
    /// `key`, if there was any.
    fn insert(&mut self, key: usize, value: Self::TB) -> Option<Self::TB>;

    /// Remove an element from the collection, returning it if it was present.
    fn remove(&mut self, key: usize) -> Option<Self::TB>;

    /// Get a reference to an element in the collection.
    fn get(&self, key: usize) -> Option<&Self::TB>;

    /// Get a mutable reference to an element in the collection.
    fn get_mut(&mut self, key: usize) -> Option<&mut Self::TB>;

    /// Get mutable references to two distinct elements.
    ///
    /// # Panics
    /// Panics if the two references point to the same object, i.e., if `key_a` =
    /// `key_b`.
    fn get_two_mut(
        &mut self,
        key_a: usize,
        key_b: usize,
    ) -> Option<(&mut Self::TB, &mut Self::TB)>;

    /// Get the number of elements in the collection.
    fn len(&self) -> usize;

    /// Check whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Collections that can be iterated over.
// instead of requiring that &T and &mut T implement IntoIterator, we have the iter and
// iter_mut methods, respectively; the reason is that having the additional bounds would
// either need an annoying lifetime or HRTBs, the latter would limit the use cases of
// the trait (for <'l> &'l T implies T: 'static); implementors of this type should
// probably still implement IntoIterator for its references (they can just call this
// function here (or vice versa))
pub trait Iterable {
    /// The type of the elements in the collection
    ///
    /// This type should be equal to [Base::TB] and [IterableBase::T], if they are
    /// implemented.
    type TI;

    /// An iterator over the collection. The items are tuples of the keys and references
    /// to the corresponding elements.
    type Iter<'l>: Iterator<Item = (usize, &'l Self::TI)>
    where
        Self: 'l;

    /// An iterator over the collection. The items are tuples of the keys and mutable
    /// references to the corresponding elements.
    type IterMut<'l>: Iterator<Item = (usize, &'l mut Self::TI)>
    where
        Self: 'l;

    /// Get an [Iterator] over the tuples of keys and references of the corresponding
    /// elements.
    fn iter_pairs(&self) -> Self::Iter<'_>;

    /// Get an [Iterator] over the tuples of keys and mutable references of the
    /// corresponding elements.
    fn iter_pairs_mut(&mut self) -> Self::IterMut<'_>;

    /// Sort the collection according to the keys.
    fn sort_by_key(&self) -> Vec<(usize, &Self::TI)> {
        let mut ret = self.iter_pairs().collect::<Vec<(usize, &Self::TI)>>();
        ret.sort_by_key(|(i, _)| *i);
        ret
    }
}

/// A helper trait for easier initialization.
pub trait Init {
    /// Initialize the collection to keep `len` many elements, with keys/indices from 0
    /// to `len` - 1.
    ///
    /// # Example
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::collection::{Init, Base, MappedVector};
    /// let collection = MappedVector::<i32>::init(2);
    /// assert_eq!(collection.get(0), Some(&0));
    /// assert_eq!(collection.get(1), Some(&0));
    /// assert_eq!(collection.get(2), None);
    /// # }
    /// ```
    fn init(len: usize) -> Self;
}

/// A superset of [Base] and [Iterable].
pub trait IterableBase: Base<TB = Self::T> + Iterable<TI = Self::T> {
    /// The type of the elements in the collection
    ///
    /// This type should be equal to [Base::TB] and [Iterable::TI], if they are
    /// implemented.
    type T;
}

/// A superset of [IterableBase], [Init] and some other standard traits.
pub trait Full:
    IterableBase
    + Init
    + IntoIterator<Item = (usize, Self::TI)>
    + FromIterator<(usize, Self::TI)>
{
    /// Convert the collection into a sorted [Vec] according to the keys/indices.
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
mod naive_vector;

pub use buffered_vector::BufferedVector;
pub use map::Map;
pub use mapped_vector::MappedVector;
pub use naive_vector::NaiveVector;
