// lints and similar
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
// opting out is the exception
#![warn(missing_copy_implementations)]
// semantically wrong; but useful for init stuff, cf. comments below
// imagine #![warn(missing_default_implementations)]
//
// (nightly) features, only for development
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// cf .https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html (I
// thought doc-test should capture the normal #! attributes?)
#![cfg_attr(coverage_nightly, doc(test(attr(feature(coverage_attribute)))))]
//

// some guidelines (should do a better contributing file ...):
//
// If possible all structs and enums should derive
// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// in this order! The fixed order, so that it is easier to see if something is missing.
// If a trait cannot be derived and it makes sense to implement it, or we need some
// custom implementation, do it manually. The same thing is valid for Serialize and
// Deserialized, conditioned by a cfg(_attr)(feature = "serde"(, ...)).
//
// Debug, Clone and Default have to be implemented, except if it is possible (e.g.,
// Default is not really possible if the type contains references). Default is
// debatable, because it doesn't make always sense, semantically, but it is useful for
// initialization; annotate such cases with with #[doc = non_semantic_default!()].
//
// All types must implement Copy, except if they are really not Copy
//
// set up all feature code as follows for proper feature documentation: #[cfg(feature =
// "<feature>")] #[cfg_attr(docsrs, doc(cfg(feature = "<feature>")))] --cfg docsrs is
// set when the documentation is build
//
// the lines of the tests should not be included in the coverage, therefore, put
// #[cfg_attr(coverage_nightly, coverage(off))] on every test function (except if the test
// is ignore, e.g., proptest); also on closures (except if we are in a doc-test and it
// is is a oneline closure in, for example, iter::map, and adding the annotation would
// change the formatting) and functions that are exclusively used in the test (except we
// really want coverage for them); this attribute does sadly not work with modules; to
// make things easier one can `use coverage_helper::test` in the test modules and just
// use the (modified) #[test] attribute; in doc-tests we always need to specify the main
// function explicitly and put the ...coverage(off)... attribute on it
//
// tests are always run with --all--features; however, doc-tests should be under
// cfg-features conditions if they use them (and this should also be documented) and
// should be tested with only those features enabled
//
// When defining a new type, add it to the marker test at the end of this file (with a
// customized check function if required). If this test fails in the future, it would be
// a breaking change!

//
#![doc = include_str!("../xdocs/lib.md")]

macro_rules! non_semantic_default {
    () => {
        "Note that semantically, this impl makes not much sense. It is rather useful for \
         initialization."
    };
}

pub mod boolean_vector;

#[cfg(feature = "circuit")]
#[cfg_attr(docsrs, doc(cfg(feature = "circuit")))]
pub mod circuit;

pub(crate) mod clifford_helper;

pub mod collection;

#[deprecated(
    since = "0.3.2",
    note = "use the `scheduler` module from the mbqc_scheduling library in the \
    [mbqc_scheduling project]\
    (https://github.com/taeruh/mbqc_scheduling/tree/main/mbqc_scheduling)"
)]
#[cfg(feature = "scheduler")]
#[cfg_attr(docsrs, doc(cfg(feature = "scheduler")))]
pub mod scheduler;

pub mod pauli;

mod slice_extension;

pub mod tracker;

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn marker() {
        // cf. "List of all items" in docs
        // Structs
        normal::<boolean_vector::bitvec_simd::Iter>();
        normal::<boolean_vector::bitvec_simd::IterFromRef>();
        normal::<boolean_vector::bitvec_simd::SimdBitVec>();
        normal::<circuit::DummyCircuit>();
        normal::<circuit::RandomMeasurementCircuit>();
        normal::<circuit::TrackedCircuit<(), (), ()>>();
        normal::<collection::BufferedVector<()>>();
        normal::<collection::MappedVector<()>>();
        normal::<pauli::PauliDense>();
        normal::<pauli::PauliStack<()>>();
        normal::<pauli::PauliTuple>();
        normal::<pauli::stack::BitCharError>();
        normal::<tracker::MissingBit>();
        normal::<tracker::frames::Frames<()>>();
        normal::<tracker::frames::OverwriteStack<()>>();
        normal::<tracker::live::Live<()>>();
        // Enums
        normal::<pauli::PauliEnum>();
        normal::<tracker::frames::MoveError<()>>();
    }
}
