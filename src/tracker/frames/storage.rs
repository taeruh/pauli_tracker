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
