//! A library to track Pauli frames through a Clifford circuit with measurements. A
//! general introduction to this library is provided in the
//! [README](https://github.com/taeruh/pauli_tracker).
//!
//! *more documentation and functionality coming soon*
//! # Crate features
//! * **circuit**
//!   Includes the [circuit] module which contains tools to combine the Pauli tracking
//!   mechanism with a circuit simulator/description.
//! * **serde**
//!   Support [serde](https://docs.rs/serde/latest/serde/) for the structs in [tracker].

#![cfg_attr(docsrs, feature(doc_cfg))]
//-
// #![warn(missing_docs)] // turn on when things are more stable
#![deny(unsafe_op_in_unsafe_fn)]

// set up all feature code as follows (for proper documentation):
// #[cfg(feature = "<feature>")]
// #[cfg_attr(docsrs, doc(cfg(feature = "<feature>")))]

pub mod bool_vector;

#[cfg(feature = "circuit")]
#[cfg_attr(docsrs, doc(cfg(feature = "circuit")))]
#[allow(unused)]
pub mod circuit;

pub mod pauli;

mod slice_extension;

pub mod tracker;

// /// Figure out which target feature has been enabled regarding SIMD operations. For
// /// example, if avx2 has been enabled, we probably have the most efficient
// /// implementation of "simd-types". These features should be automatically
// /// enabled at compile time if they are available on the compiled architecture
// #[allow(unreachable_code)] // because rust-analyzer detects the target_feature(s)
// pub fn enabled_target_feature() -> &'static str {
//     #[cfg(target_feature = "avx2")]
//     {
//         return "avx2";
//     }
//     #[cfg(target_feature = "sse2")]
//     {
//         return "sse2";
//     }
//     "none"
// }
