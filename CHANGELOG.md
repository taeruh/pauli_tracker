# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]
### Added
- **Possible Breaking change**; Implement/derive `Debug` for `DummyCircuit` and
  `RandomMeasurementCircuit`
- **Possible Breaking change**; Implement `From<Vec<Pauli>> for LiveVector`
- Add `DependencyGraph` type
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [0.1.1] - 2023-06-01
### Added
- Add the `circuit` module
### Fixed
- Fix bug in the movement operations for the live tracker

## [0.1.0] - 2023-06-01

[Unreleased]: https://github.com/taeruh/pauli_tracker/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/taeruh/pauli_tracker/compare/v0.1.0...v0.1.1
