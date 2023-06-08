//! A library to track Pauli frames through a Clifford circuit with measurements. A
//! general introduction to this library is provided in the
//! [README](https://github.com/taeruh/pauli_tracker).
//!
//! *more documentation, especially examples, are in progress*
//!
//! # Crate features
//! * **serde**
//!   Support [serde](https://docs.rs/serde/latest/serde/) for custom types.
//! * **circuit**
//!   Includes the [circuit] module which contains tools to combine the Pauli tracking
//!   mechanism with a circuit simulator/description.
//! * **bitvec**
//!   Implement [BooleanVector](boolean_vector::BooleanVector) for
//!   [bitvec::vec::BitVec](https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html)
//!   (extern crate).
//! * **bit-vec**
//!   Implement [BooleanVector](boolean_vector::BooleanVector) for
//!   [bit_vec::BitVec](https://docs.rs/bit-vec/latest/bit_vec/struct.BitVec.html)
//!   (extern crate).
//! * **bitvec_simd**
//!   Implement [BooleanVector](boolean_vector::BooleanVector) for wrapper
//!   [SimdBitVec](boolean_vector::bitvec_simd::SimdBitVec) around
//!   [bitvec_simd::BitVec](https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html)
//!   (extern crate). Note that while this bit-vector implementation uses SIMD
//!   operations (if available), it also uses the crate
//!   [smallvec](https://docs.rs/smallvec/1.10.0/smallvec/) for its inner storage. That
//!   may be not memory efficient for the Pauli tracking since the storage is fairly
//!   big.

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
