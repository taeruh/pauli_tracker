/*!
A library to track Pauli frames through a Clifford circuit with measurements. A general
introduction to this library is in the
[README](https://github.com/taeruh/pauli_tracker) provided.
# Crate features
### Stable features
* **circuit**
  Includes the [circuit] module which contains tools to combine the Pauli tracking
  mechanism with a circuit simulator/description.
### Nightly features
* **not-nightly**
  This feature is exclusive with all other nightly features. This enables one to
  use the flag `--all-features` on stable toolchain without without enabling the nightly
  features. However, this also implies that using `--all-features` on the nightly
  toolchain does not include the nightly features.
* **doc-build-nightly**
  Build the docs with additional nightly doc-features, e.g., annotating modules with
  features.
*/

#![cfg_attr(
    all(feature = "doc-build-nightly", not(feature = "not-nightly")),
    feature(doc_cfg)
)]
//-
// #![warn(missing_docs)] // turn on when things are more stable
#![deny(unsafe_op_in_unsafe_fn)]

// set up all feature code as follows (for proper documentation):
// #[cfg(any(feature = "<feature>", doc))]
// #[cfg_attr(
//     all(feature = "doc-build-nightly", not(feature = "not-nightly")),
//     doc(cfg(feature = "<feature>"))
// )]

#[cfg(any(feature = "circuit", doc))]
#[cfg_attr(
    all(feature = "doc-build-nightly", not(feature = "not-nightly")),
    doc(cfg(feature = "circuit"))
)]
pub mod circuit;
pub mod pauli;
mod slice_extension;
pub mod tracker;

/// Figure out which target feature has been enabled regarding SIMD operations. For
/// example, if avx2 has been enabled, we probably have the most efficient
/// implementation of "simd-types". These features should be automatically
/// enabled at compile time if they are available on the compiled architecture
#[allow(unreachable_code)] // because rust-analyzer detects the target_feature(s)
pub fn enabled_target_feature() -> &'static str {
    #[cfg(target_feature = "avx2")]
    {
        return "avx2";
    }
    #[cfg(target_feature = "sse2")]
    {
        return "sse2";
    }
    "none"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Check whether the correct target feature is enabled (in the build script or
    // automatically)
    fn target_feature() {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx2") {
            assert_eq!("avx2", enabled_target_feature());
        } else if is_x86_feature_detected!("sse2") {
            assert_eq!("sse2", enabled_target_feature());
        }
    }
}
