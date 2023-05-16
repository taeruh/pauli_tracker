#![cfg_attr(all(feature = "doc-build", not(feature = "not-nightly")), feature(doc_cfg))]
//-
// #![warn(missing_docs)] // turn on when things are more stable
#![deny(unsafe_op_in_unsafe_fn)]
//-
#![doc = include_str!("../README.md")]

pub mod circuit;
