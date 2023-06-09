/*!
The underlining storage types for [Frames](super::Frames) and some functionality to
analyze the storage.
*/

use crate::{
    boolean_vector::BooleanVector,
    pauli::PauliVec,
};

/// This trait describes the functionality that a storage of [PauliVec]s must provide to
/// be used as storage for [Frames](super::Frames).
// instead of requiring that &T and &mut T implement IntoIterator, we have the iter and
// iter_mut methods, respectively; the reason is that having the additional bounds would
// either need an annoying lifetime or HRTBs, the latter would limit the use cases of
// the trait (for <'l> &'l T implies T: 'static); implementors of this type should
// probably still implement IntoIterator for its references
pub trait StackStorage:
    IntoIterator<Item = (usize, PauliVec<Self::BoolVec>)>
    + FromIterator<(usize, PauliVec<Self::BoolVec>)>
{
    /// The storage type used for [PauliVec].
    type BoolVec: BooleanVector;

    /// An iterator over the storage. The items are tuples of the qubits with references
    /// to their corresponding [PauliVec] stack.
    type Iter<'l>: Iterator<Item = (usize, &'l PauliVec<Self::BoolVec>)>
    where
        Self: 'l;

    /// An iterator over the storage. The items are tuples of the qubits with mutalbe
    /// references to their corresponding [PauliVec] stack.
    type IterMut<'l>: Iterator<Item = (usize, &'l mut PauliVec<Self::BoolVec>)>
    where
        Self: 'l;

    /// Initialize the storage to keep `num_bits` Pauli stacks, numbered from 0 to
    /// `num_bits` - 1.
    ///
    /// # Example
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// use pauli_tracker::{
    ///     pauli::PauliVec,
    ///     tracker::frames::storage::{
    ///         Map,
    ///         StackStorage,
    ///     },
    /// };
    /// let storage = Map::<Vec<bool>>::init(2);
    /// assert_eq!(storage.get(&0), Some(&PauliVec::<Vec<bool>>::new()));
    /// assert_eq!(storage.get(&1), Some(&PauliVec::<Vec<bool>>::new()));
    /// assert_eq!(storage.get(&2), None);
    /// # }
    /// ```
    fn init(num_bits: usize) -> Self;

    /// Check whether the storage is empty.
    fn is_empty(&self) -> bool;

    /// Insert a `pauli` stack for qu`bit`. If the qu`bit` is already present,
    /// its stack is overwritten an the old value, [Some](Some)(\<old stack\>), is
    /// returned.
    fn insert_pauli(
        &mut self,
        bit: usize,
        pauli: PauliVec<Self::BoolVec>,
    ) -> Option<PauliVec<Self::BoolVec>>;

    /// Remove a qu`bit` and its stack from the storage. If the qubit is present, its
    /// stack is returneddd, otherwise [None].
    fn remove_pauli(&mut self, bit: usize) -> Option<PauliVec<Self::BoolVec>>;

    /// Get a references to qu`bit`s stack, if present, otherwise return [None].
    fn get(&self, bit: usize) -> Option<&PauliVec<Self::BoolVec>>;

    /// Get a mutalbe references to qu`bit`s stack, if present, otherwise return [None].
    fn get_mut(&mut self, bit: usize) -> Option<&mut PauliVec<Self::BoolVec>>;
    #[allow(clippy::type_complexity)]

    /// Get two mutable references to distinct elements.
    ///
    /// # Panics
    /// Panics if the two references point to the same object, i.e., if `bit_a` =
    /// `bit_b`.
    fn get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> Option<(&mut PauliVec<Self::BoolVec>, &mut PauliVec<Self::BoolVec>)>;

    /// Get an [Iterator] over the tuples of qubits and references of the corresponding
    /// Pauli stacks.
    fn iter(&self) -> Self::Iter<'_>;

    /// Get an [Iterator] over the tuples of qubits and mutable references of the
    /// corresponding Pauli stacks.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Sort the `storage` according to the qubits numbers.
pub fn sort_by_bit<B: StackStorage>(
    storage: &B,
) -> Vec<(usize, &PauliVec<B::BoolVec>)> {
    let mut ret = storage.iter().collect::<Vec<(usize, &PauliVec<B::BoolVec>)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

/// Convert the `storage` into a sorted array according to the qubits numbers.
pub fn into_sorted_by_bit<B: StackStorage>(
    storage: B,
) -> Vec<(usize, PauliVec<B::BoolVec>)> {
    let mut ret = storage.into_iter().collect::<Vec<(usize, PauliVec<B::BoolVec>)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

mod vector;
pub use vector::Vector;

mod map;
pub use map::Map;

mod mapped_vector;
#[allow(unused)] // we're using it in some tests
pub(crate) use mapped_vector::MappedVector;

#[cfg(test)]
mod tests {
    // use coverage_helper::test;

    // // First we test the methods of [FullMap] that are not just simple redirections.
    // // Then we use [FullMap] to as reference to test the other storages

    // #[test]
    // fn full_map() {
    //     /* all trivial */
    // }

    // #[test]
    // fn mapped_vec() {
    //     // do some fuzzing using dispatch_storage_operation_comparison below
    // }
}

// #[cfg(test)]
// fn dispatch_storage_operation_comparison(
//     storage: &mut (impl PauliStorage + PartialEq + Clone),
//     other: &mut FullMap,
//     operation: u8,
//     bit: usize,
// ) {
//     let operation = operation % 3;
//     match operation {
//         0 => {
//             assert_eq!(
//                 storage.insert_pauli(bit, PauliVec::new()),
//                 other.insert_pauli(bit, PauliVec::new())
//             );
//         }
//         1 => {
//             assert_eq!(storage.remove_pauli(bit), other.remove_pauli(bit));
//         }
//         2 => {
//             assert_eq!(storage.get(bit), other.get(&bit));
//         }
//         _ => {}
//     }
//     let compare = FullMap::from_iter(storage.clone().into_iter());
// }
