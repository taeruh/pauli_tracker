//! A library to track Pauli frames through a Clifford circuit with measurements. A
//! general brief introduction to Pauli tracking is given in the repository's
//! [README](https://github.com/taeruh/pauli_tracker).
//!
//! *more documentation, especially examples, are in progress*
//!
//! # Crate features
//!
//! * **serde**
//!   Support [serde](https://docs.rs/serde/latest/serde/) for custom types.
//! * **circuit**
//!   Includes the [circuit] module which contains tools to combine the Pauli tracking
//!   mechanism with a circuit simulator/description.
//! * **bitvec**
//!   Implement [BooleanVector](boolean_vector::BooleanVector) for
//!   [bitvec::vec::BitVec](https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html)
//!   (extern crate). Note that we do not export any types of
//!   [bitvec](https://docs.rs/bitvec/latest/bitvec/index.html); you need to depend on
//!   it manually to use its types.
//! * **bit-vec**
//!   Implement [BooleanVector](boolean_vector::BooleanVector) for
//!   [bit_vec::BitVec](https://docs.rs/bit-vec/latest/bit_vec/struct.BitVec.html)
//!   (extern crate). Note that we do not export any types of
//!   [bit-vec](https://docs.rs/bit-vec/latest/bit_vec/index.html); you need to depend
//!   on it manually to use its types.
//! * **bitvec_simd**
//!   Implement [BooleanVector](boolean_vector::BooleanVector) for wrapper
//!   [SimdBitVec](boolean_vector::bitvec_simd::SimdBitVec) around
//!   [bitvec_simd::BitVec](https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html)
//!   (extern crate). Note that while this bit-vector implementation uses SIMD
//!   operations (if available), it also uses the crate
//!   [smallvec](https://docs.rs/smallvec/1.10.0/smallvec/) for its inner storage. That
//!   may be not memory efficient for the Pauli tracking since the storage is fairly
//!   big.
//!
//! # Examples
//!
//! ### A first idea
//!
//! This example requires the [rand](https://crates.io/crates/rand) crate.
//! ```
//! #[rustfmt::skip]
//! use pauli_tracker::{
//!     tracker::{Tracker, live::LiveVector, frames::{storage, Frames}},
//!     pauli::{self, Pauli},
//! };
//!
//! // first, we will use the Frames tracker to fully track all Pauli frames
//!
//! // the Frames tracker is generic over its storage types, which themselves are
//! // generic; it's almost always sensible to specific define types
//! type BoolVec = Vec<bool>; // you might want to use a "bit-vector"; cf. features
//! type Storage = storage::Map<BoolVec>;
//! type PauliVec = pauli::PauliVec<BoolVec>;
//!
//! // initialize the tracker with three qubits
//! let mut tracker = Frames::<Storage>::init(3);
//!
//! // track Paulis through an (imaginary) circuit
//! // X(0), CX(0, 1), S(1), Z(2), CZ(1, 2), H(0)
//! tracker.track_pauli(0, Pauli::new_x()); // track a new Pauli X; frame (0)
//! tracker.cx(0, 1); // conjugate with a Control X gate
//! tracker.s(1); // conjugate with an S gate
//! tracker.track_pauli(2, Pauli::new_y()); // track a new Pauli Z; frame (1)
//! tracker.cz(1, 2); // conjugate with a Control Z gate
//! tracker.h(0); // conjugate with an H gate
//!
//! // let's get the frames (sorted into a Vec for convenience)
//! let frames = storage::into_sorted_by_bit(tracker.into_storage());
//!
//! // what would we expect (calculate it by hand)?
//! let mut expected =
//!     vec![(0, PauliVec::new()), (1, PauliVec::new()), (2, PauliVec::new())];
//! // {{ frame (0)
//! expected[0].1.push(Pauli::new_z());
//! expected[1].1.push(Pauli::new_y());
//! expected[2].1.push(Pauli::new_z());
//! // }}
//! // {{ frame (1)
//! expected[0].1.push(Pauli::new_i());
//! expected[1].1.push(Pauli::new_z());
//! expected[2].1.push(Pauli::new_y());
//! // }}
//! // (creating `expected` can be faster achieved with PauliVec::try_from_str, e.g., as
//! // in the example "The dependency graph")
//!
//! // let's check it
//! assert_eq!(frames, expected);
//!
//! // let's vary the example from above a little bit: Paulis are often induced as
//! // corrections in MBQC; these corrections might effect the measurement basis of
//! // following measurements; to get the final correction before a measurement we could
//! // add the frames in `frames` from above, however, we can also do it directly with
//! // the LiveTracker:
//!
//! let mut tracker = LiveVector::init(3); // initialize the tracker with three qubits
//!
//! // a small helper to track Paulis conditioned on measurements (the circuit module
//! // provides similar helpers)
//! let mut measurements = Vec::<bool>::new();
//! let mut correct = |tracker: &mut LiveVector, bit, pauli| {
//!     // "measurement"; in a real use case this would be, for example, a quantum
//!     // measurement
//!     let outcome = rand::random::<bool>();
//!     if outcome {
//!         tracker.track_pauli(bit, pauli);
//!     }
//!     measurements.push(outcome);
//! };
//!
//! // the same circuit from above, but with conditional Paulis, e.g., MBQC corrections
//! correct(&mut tracker, 0, Pauli::new_x());
//! tracker.cx(0, 1);
//! tracker.s(1);
//! correct(&mut tracker, 2, Pauli::new_y());
//! tracker.cz(1, 2);
//! tracker.h(0);
//!
//! // let's checkout the final corrections
//! println!("{tracker:?}");
//!
//! // we can check whether the output of the live tracker aligns with the frames
//! // tracker
//! let conditional_summed_frames: Vec<_> = frames
//!     .into_iter()
//!     .map(|(_, pauli_stack)| pauli_stack.sum_up(&measurements))
//!     .collect();
//! assert_eq!(*tracker.as_ref(), conditional_summed_frames, "{measurements:?}");
//! ```
//!
//! ### The dependency graph
//! ```
//! #[rustfmt::skip]
//! use pauli_tracker::{
//!     tracker::{Tracker, frames::{storage, Frames}},
//!     pauli::{self, Pauli},
//! };
//!
//! type BoolVec = bit_vec::BitVec;
//! // we want a fixed order in our storage for this test, so we use Vector and not Map
//! type Storage = storage::Vector<BoolVec>;
//! type PauliVec = pauli::PauliVec<BoolVec>;
//!
//! // let's consider the following tracking pattern
//!
//! let mut tracker = Frames::<Storage>::init(6);
//!
//! tracker.track_pauli(0, Pauli::new_x()); // frame (0)
//! tracker.cx(0, 1);
//! tracker.s(1);
//! tracker.track_pauli(2, Pauli::new_y()); // frame (1)
//! tracker.cz(1, 2);
//! tracker.cx(3, 2);
//! tracker.track_pauli(1, Pauli::new_z()); // frame (2)
//! tracker.h(1);
//! tracker.cx(3, 1);
//! tracker.cz(3, 2);
//!
//! // check its output
//! assert_eq!(
//!     storage::sort_by_bit(tracker.as_storage()),
//!     vec![ // tableau representation:  X      Z   ; the columns are the frames
//!         (0, &PauliVec::try_from_str("100", "000").unwrap()),
//!         (1, &PauliVec::try_from_str("111", "100").unwrap()),
//!         (2, &PauliVec::try_from_str("010", "110").unwrap()),
//!         (3, &PauliVec::try_from_str("000", "000").unwrap()),
//!         (4, &PauliVec::try_from_str("000", "000").unwrap()),
//!         (5, &PauliVec::try_from_str("000", "000").unwrap()),
//!     ]
//! );
//!
//! // now, we assume that when the real circuit is executed, the paulis are
//! // conditionally tracked on measurement outcomes of the qubits 3,4 and 0, i.e.
//! let map =  // describe the relation between frames and qubits
//!     4, // frame (0) depends on the measurement on qubit 3
//!     5, // frame (1) on the measurement on qubit 4
//!     0, // frame (1) on the measurement on qubit 0
//! ;
//!
//! // we are interested in how many steps of parallel measurement we need to measure
//! // qubits "0" to "4"; this can be figured out with the dependency graph:
//! let graph = storage::create_dependency_graph(tracker.as_storage(), &map);
//! assert_eq!(
//!     graph, // fixed order because we set Storage = Vector
//!     // the tuples consist of the qubit and its dependencies
//!     vec![
//!         vec![(3, vec![]), (4, vec![]), (5, vec![])], // layer 0
//!         vec![(0, vec![4]), (2, vec![4, 5])],         // layer 1
//!         vec![(1, vec![5, 0])],                       // layer 2
//!     ]
//! );
//! // - in layer 0, there are no Paulis before the measurements, i.e., we have no
//! //   dependecies; the qubits in layer 1 depend only on outcomes of qubits in layer
//! //   0; the qubits in layer 2 depend only on qubits in layer 0, ..., 1; and so on
//! // - the graph removes redundent dependencies, e.g., although qubit 1 depends on
//! //   [0, 4, 5] (cf. the output of the tracker above), the graph only lists [0, 5];
//! //   this is because qubit 0 already depends on the outcome of qubit 4
//! // - we see that the graph has three layers, this means that the six measurements
//! //   can be performed in 3 steps
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
//-
// #![warn(missing_docs)] // turn on when things are more stable
#![deny(unsafe_op_in_unsafe_fn)]

// set up all feature code as follows (for proper documentation):
// #[cfg(feature = "<feature>")]
// #[cfg_attr(docsrs, doc(cfg(feature = "<feature>")))]

pub mod boolean_vector;

#[cfg(feature = "circuit")]
#[cfg_attr(docsrs, doc(cfg(feature = "circuit")))]
#[allow(unused)]
pub mod circuit;

pub mod pauli;

mod slice_extension;

pub mod tracker;

/// Figure out which target feature has been enabled regarding SIMD operations.
///
///For example, if avx2 has been enabled, we probably have the most efficient
///implementation of "simd-types". Some features are automatically enabled at compile
///time and some have to be enabled manually, for example, in your `build.rs` script:
/// ```
/// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// if is_x86_feature_detected!("avx2") {
///     println!(r#"cargo:rustc-cfg=target_feature="avx2""#);
/// }
/// ```
/// ***currently this function only tests against "avx2" and "sse"***
#[allow(unreachable_code)] // because rust-analyzer detects the target_feature(s)
pub fn enabled_simd_target_feature() -> &'static str {
    #[cfg(target_feature = "avx2")]
    {
        return "avx2";
    }
    #[cfg(target_feature = "sse2")]
    {
        return "sse2";
    }
    "other or none"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check whether the code in the documentation of [enabled_target_feature] makes
    // sense (and whether we enabled the target feature that we want when running the
    // tests)
    fn target_feature() {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx2") {
            assert_eq!("avx2", enabled_simd_target_feature());
        } else if is_x86_feature_detected!("sse2") {
            assert_eq!("sse2", enabled_simd_target_feature());
        }
    }
}
