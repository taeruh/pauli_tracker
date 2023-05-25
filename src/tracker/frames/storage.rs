/// Helper trait to basically use something like [slice::get_many_mut], which is
/// currently unstable.
trait GetTwoMutSlice {
    type SliceType;

    unsafe fn get_two_unchecked_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)>;

    fn get_two_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)>;
}

// We are basically doing what std::slice does (cannot really use it because it is
// unstable at the moment), stripping down the chain of (unstable) method calls
impl<T> GetTwoMutSlice for [T] {
    type SliceType = T;

    /// # Safety
    ///
    /// The indices `one` and `two` have two different and in bounds.
    unsafe fn get_two_unchecked_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)> {
        // doing something like for the HashMap triggers miri stacked-borrow errors;
        // doing it with the pointers directly is cleaner anyway
        let ptr: *mut T = self.as_mut_ptr();
        let a = unsafe { &mut *ptr.add(one) };
        let b = unsafe { &mut *ptr.add(two) };
        Some((a, b))
    }

    fn get_two_mut(
        &mut self,
        one: usize,
        two: usize,
    ) -> Option<(&mut Self::SliceType, &mut Self::SliceType)> {
        // we could have done that using std::slice::spli_at_mut, not needing to write
        // unsafe code our own here, but I feel like the unsafe code expresses better
        // what we are actually doing and it's invariants are pretty straightforward
        let len = self.len();
        if one == two || one > len || two > len {
            return None;
        }
        // Safety: the above conditational ensures that the requirements are fulfilled
        unsafe { self.get_two_unchecked_mut(one, two) }
    }
}

mod fixed_vector;
pub use fixed_vector::FixedVector;

mod full_map;
pub use full_map::FullMap;

mod mapped_vector;
pub use mapped_vector::MappedVector;

#[cfg(test)]
mod tests {
    // use super::*;

    // First we test the methods of [FullMap] that are not just simple redirections.
    // Then we use [FullMap] to as reference to test the other storages

    #[test]
    fn full_map() {
        /* all trivial */
    }

    #[test]
    fn mapped_vec() {
        // do some fuzzing using dispatch_storage_operation_comparison below
    }
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
