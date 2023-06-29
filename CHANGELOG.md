# Changelog
All notable changes to this project will be documented in this file, as best as
possible.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

We try to follow the [SemVer](https://semver.org/) rules, specifically 
[Cargo guidelines](https://doc.rust-lang.org/cargo/reference/semver.html), as best as
possible.

## [Unreleased]
### Added
- **Breaking Change**: Add blanket `impl<S> AsRef<S> for Frames<S>`.
- Add a new `analyse`, activated by the "analyse" feature.
- **Possible Breaking Change**: Add `Frames::new`.
- **Possible Breaking Change**: Add `impl Default for SimdBitVec`.
### Changed
- **Breaking Change**: Return u8 `in Pauli::storage` instead of a reference.
- **Breaking Change**: Change the Debug and Display implementations of `Pauli`. Debug is
  now derived, so we have the standard format, and Display shows "X, Y, Z, I" instead of
  numbers.
- **Breaking Change**: In `StackStorage::insert_pauli`, overwrite and return the old
  value if the qubit is already present, not the new value.
- **Breaking Change**: Return the measurement outcomes in in
  `circuit::measure_and_store(_all)` and error if one would overwrite something in the
  additional storage.
- **Breaking Change**: Move `create_dependency_graph` into the `analyse` module
- **Possible Breaking Change**: Implement Clone, PartialEq, Debug for
  `bitvec_simd::Iter*`; Implement Debug for TrackedCircuit.
- For the "fixed" `Vector`, it wasn't allowed to `insert_pauli`s at qubits which have a
  higher number than the length of the inner storage type (it would panic). Now it is
  allowed, but some buffer stacks are inserted.
- **Breaking Change**: All functions that returned Results with Strings as Err, now
  return real Error types.
- **Breaking Change**: `TrackedCircuit::measure_and_store*` now return a tuple of the
  measurement outcome and the storing result. Before, they returned a result, where the
  Ok contained the measurement outcomes and Err the storing error.
- **Breaking Change**: Add `FromIterator` as supertrait to `StackStorage`.
### Deprecated
### Removed
### Fixed
- Now `Map`s `StackStorage::insert_pauli` actually does what the trait's signature
  documentation says.
- Fix `impl FromIterator<bool> for SimdBitVec`; it was not working at all.
### Security

## [0.2.2] + 2023-06-23
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

[Unreleased]: https://github.com/taeruh/pauli_tracker/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/taeruh/pauli_tracker/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taeruh/pauli_tracker/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taeruh/pauli_tracker/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/taeruh/pauli_tracker/compare/v0.1.0...v0.1.1
