/*!
A library to track Pauli frames through a Clifford circuit with measurements. A general
introduction to this library is in the
[README](https://github.com/taeruh/pauli_tracker) provided.
# Crate features
### Stable features
* **circuit**
  Includes the [circuit] module which contains structs and methods to describe certain
  quantum circuits.
### Nightly features
* **not-nightly**
  This feature is exclusive with all other nightly features here. This enables one to
  use the flag `--all-features` when on stable toolchain. However, this also implies
  that using `--all-features` when on the nightly toolchain does not include the nightly
  features.
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

#[cfg(any(feature = "circuit", doc))]
#[cfg_attr(
    all(feature = "doc-build-nightly", not(feature = "not-nightly")),
    doc(cfg(feature = "circuit"))
)]
pub mod circuit;
