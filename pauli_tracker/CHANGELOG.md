# Changelog

All notable changes to this project will be documented in this file, as best as possible.

The format is based on [Keep a Changelog].

We (try to) follow the [SemVer] rules, specifically [Cargo guidelines].

## [Future major bump]
### Added
### Changed
**Breaking Change**: Remove the default implementations for the `Tracker::remove_*`
methods.
### Deprecated
### Removed
- **Breaking Change**: Remove `Pauli::add` in favour of `Pauli::multiply`.
### Fixed
### Security

## [Unreleased]
### Added
### Changed
### Deprecated
- Deprecate the `Pauli::add` method in favour of `Pauli::multiply`.
### Removed
### Fixed
- Fix the documentation of `Pauli::(xpz|zpx)` (was flipped).
- Fix `From<PauliTuple> for PauliEnum`
### Security

## [0.4.1] - 2024-05-07
### Added
- Add the `Tracker::remove_*` methods (with a panicking default implementation).
- Add the `Tracker::zc*` methods.
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [0.4.0] - 2024-02-12
### Added
- Add `Frames::stacked_transpose`.
- Add `Frames::get_frame`.
- Add `From<(bool, bool)> for PauliTuple`.
### Changed
- **Breaking Change**: Remove `Frames::transpose_reverted` and add
  `Frames::stacked_transpose` instead.
- **Breaking Change**: Rename "left/right" to "x/z".
- **Breaking Change**: Swapped entries int `PauliTuple`.
- **Breaking Change**: Swapped args in `Pauli::new_product` and
`PauliStack::try_from_str`.
- **Breaking Change**: Rename `dependency_graph` to `induced_order`,
`create_dependency_graph` to `get_order` and `DependencyGraph` to `PartialOrderGraph`.
### Deprecated
### Removed
- **Breaking Change**: Remove the `scheduler` module.
- **Breaking Change**: Remove `BufferedVector::wrap` and `NaiveVector::wrap` (use `from`
instead), as well as `PathGenerator::measureable` (use `measurable` instead). `Pauli::sx`
and `PauliStack::sx` (use `shs` instead).
### Fixed
### Security

## [0.3.2] - 2024-01-16
### Added
- Add `Live::as_storage`.
- Add `Frames::get`.
### Changed
### Deprecated
- `Live::into` (use `into_storage` instead)
### Removed
### Fixed
- `Live::measure` now actually removes the qubit instead of just cloning it (for the
old effect of `measure`, do something like `get(...).copied`). 
### Security

## [0.3.1] - 2023-10-10
### Added
- Add `GraphBuffer::from_sparse`.
- Add a bunch of gates.
- Add `Frames::transpose_reverted`, `PauliStack::get`, `PauliStack::get_with_default`.
### Changed
- **Possible Breaking Change**: Remove `FromIterator<PauliDense> for PauliStack<impl
  BooleanVector>` and add `FromIterator<impl Pauli> for PauliStack<impl BooleanVector>`.
- Change the canonical coset representatives for the gates (cf. `Tracker` doc).
### Deprecated
- `BufferedVector::wrap` and `NaiveVector::wrap` (use `from` instead).
- `PathGenerator::measureable` (use `measurable` instead).
- `Pauli::sx` and `PauliStack::sx` (use `shs` instead).
### Removed
### Fixed
### Security

## [0.3.0] - 2023-08-11
There's a fairly big number of breaking changes. I'm sure that I'm not listing all of
them, but hopefully the most important ones.
### Added
- **Breaking Change**: Add blanket `impl<S> AsRef<S> for Frames<S>`.
- Add a new `scheduler` module, activated by the "scheduler" feature.
- **Possible Breaking Change**: Add `Frames::new`.
- **Possible Breaking Change**: Add `impl Default for SimdBitVec`.
- **Possible Breaking Change**: Add more Clifford gates (sdg, swap, sx, sy, sz, sxdg,
  sydg, szdg, x, y, z)
### Changed
- **Breaking Change**: Rename `LiveVector` to `Live` and make it generic.
- **Breaking Change**: Use `hashbrown::HashMap` instead of `std::collections::HashMap`.
- **Breaking Change**: `Tracker::new_qubit` now overwrites the old value, and returns
  it.
- **Breaking Change**: Rename `PauliVec` to `PauliStack`.
- **Breaking Change**: Introduce a trait `crate::pauli::Pauli` to represent single
  Paulis. The former type `crate::pauli::single::Pauli` is replaced by the type
  `crate::pauli::dense::PauliDense` (which implements `Pauli`). The `Tracker` impl of
  `Frames` now uses `crate::pauli::tuple::PauliTuple` for methods like `track_Pauli`.
  `PauliTuple` and `PauliEnum` are another implementors of `Pauli`.
- **Breaking Change**: Return u8 `in PauliDense::storage` instead of a reference.
- **Breaking Change**: Change the Debug and Display implementations of `PauliDense`.
  Debug is now derived, so we have the standard format, and Display shows "X, Y, Z, I"
  instead of numbers.
- Completely refactor the `tracker::frames::storage` module: The `StackStorage` trait
  has been removed, instead there are now five traits `Base`, `Iterable`,
  `IterableBase`, `Init` and `Full` in the `collection` module which basically do the
  same job. However, they are "more generic" so that they can also be used for the
  `Live` tracker and they the methods are split up. `Base` is the absolute minimum
  that is required for the `Live` tracker. `IterableBase`, which is `Base + Iterable` is
  required for the `Frames` tracker. `Init` is just a helper for initialization and
  `Full` just the combination of all those traits. The implementors of the `collection`
  traits are located in the `collection` module. `Vector` is renamed to
  `BufferedVector`, has changed semantics and does not implement the `Deref(Mut)`
  anymore.
- **Breaking Change**: In `Base/StackStorage::insert_pauli`, overwrite and return
  the old value if the qubit is already present, instead of returning the new value.
- **Breaking Change**: Return the measurement outcomes in in
  `circuit::measure_and_store(_all)` and error if one would overwrite something in the
  additional storage.
- **Breaking Change**: Move `create_dependency_graph` and related functions into
  `frames::dependency_graph`
- **Breaking Change**: Move `sort_by_bit` and `into_sorted_by_bit` into the
  `collection` traits, replacing "bit" by "key".
- **Possible Breaking Change**: Implement a bunch of standard traits (Clone, Default,
  PartialEq, ...) for mutiple types.
- For the `BufferedVector` (previously `Vector`), it wasn't allowed to `insert_pauli`s
  at qubits which have a higher number than the length of the inner storage type (it
  would panic). Now it is allowed, but some buffer stacks are inserted.
- **Breaking Change**: All functions that returned Results with Strings as Err, now
  return real Error types.
- **Breaking Change**: `TrackedCircuit::measure_and_store*` now returns a tuple of the
  measurement outcome and the storing result. Before, it returned a result, where the
  Ok variant contained the measurement outcomes and Err the storing error.
### Deprecated
### Removed
### Fixed
- Now `Map`s `Base/StackStorage::insert_pauli` actually does what the trait's
  signature documentation says.
- Fix `impl FromIterator<bool> for SimdBitVec`; it was not working at all.
### Security

## [0.2.2] - 2023-06-23
### Added
- Add `sort_layers_by_bits` function.
- Add `pauli::encoding` constants.
- **Possible Breaking Change**: Derive Clone, PartialEq, Eq, PartialOrd, Ord for
  `storage::Vector`.
### Fixed
- Panic in `create_dependency_graph` if the input doesn't make sense, instead of
  endlessly looping.
- In `PauliVec::push`, if the left (Z) and the right (X) stacks don't have
  the same length, fill the shorter one with `false/0` so that they have the same
  length, before pushing.
- Fix `LiveVector::new_qubit` when the index is bigger then the length.

## [0.2.1] - 2023-06-14
### Added
- **Possible Breaking Change**: Add `impl BooleanVector for Vec<bool>`.
- **Possible Breaking Change**: Add `impl<T: BooleanVector> FromIterator<Pauli> for
  PauliVec<T>`.
- **Possible Breaking Change**: Add `TrackedCircuit::measure_and_store_all`.
- **Possible Breaking Change**: Add `Tracker::track_$pauli` methods where $pauli = x, y,
  z.
### Changed
- Make `Vector`s inner field public.
### Removed
- Make the `bit_vec` module private (had no public items).
### Fixed

## [0.2.0] - 2023-06-09
### Added
- **Possible Breaking Change**: Implement/derive `Debug` for `DummyCircuit` and.
  `RandomMeasurementCircuit`
- **Possible Breaking Change**: Implement `From<Vec<Pauli>> for LiveVector` and vice
  versa and `AsRef<Vec<Pauli>> for LiveVector`.
- Add `DependencyGraph` type.
- Add `enabled_simd_target_feature` function.
- **Possible Breaking Change**: Add `BooleanVector::sum_up` with a default
  implementation.
- **Possible Breaking Change**: Add `PauliVec::sum_up` method.
### Changed
- **Breaking Change**: Refactor, specifically `PauliVec`
  `PauliVec` is now generic over its "Vec" type which has to implement the new
  `BooleanVector` trait from the `boolean_vector` module (some implementors are
  provided, based on extern libraries). The refactor also includes a new structering of
  some modules and some other breaking changes around `PauliVec`. Some of them are:
  - Add associated type `BoolVec` to `StackStorage`.
  - Make the storage types in `storage` generic.
  - Move `PauliVec` into its own module.
### Fixed
- **Breaking Change**: Fix the logic behind `PauliVec::pop_or_false` -> `PauliVec::pop`.

## [0.1.1] - 2023-06-02
### Added
- Add the `circuit` module.
### Fixed
- Fix bug in the movement operations for the live tracker.

## [0.1.0] - 2023-06-01

[Unreleased]: https://github.com/taeruh/pauli_tracker/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/taeruh/pauli_tracker/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/taeruh/pauli_tracker/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/taeruh/pauli_tracker/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/taeruh/pauli_tracker/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/taeruh/pauli_tracker/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/taeruh/pauli_tracker/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taeruh/pauli_tracker/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taeruh/pauli_tracker/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/taeruh/pauli_tracker/compare/v0.1.0...v0.1.1

[Cargo guidelines]: https://doc.rust-lang.org/cargo/reference/semver.html
[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[SemVer]: https://semver.org/
