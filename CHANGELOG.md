# Changelog
All notable changes to this project will be documented in this file as best as possible.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

We try to follow the [SemVer](https://semver.org/) rules, specifically 
[Cargo guidelines](https://doc.rust-lang.org/cargo/reference/semver.html), as best as
possible.

## [Unreleased]
### Added
### Changed
### Deprecated
### Removed
- Make the `bit_vec` module private (had no public items).
### Fixed
### Security

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

[Unreleased]: https://github.com/taeruh/pauli_tracker/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/taeruh/pauli_tracker/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/taeruh/pauli_tracker/compare/v0.1.0...v0.1.1
