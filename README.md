# Pauli Tracker

[![Crates.io](https://img.shields.io/crates/v/pauli_tracker.svg)](https://crates.io/crates/pauli_tracker)
[![Documentation](https://docs.rs/pauli_tracker/badge.svg)](https://docs.rs/pauli_tracker/)
[![Build
status](https://github.com/taeruh/pauli_tracker/workflows/ci/badge.svg)](https://github.com/taeruh/pauli_tracker/actions)
[![Codecov](https://codecov.io/github/taeruh/pauli_tracker/coverage.svg?branch=main)](https://codecov.io/gh/taeruh/pauli_tracker)
[![Dependency status](https://deps.rs/repo/github/taeruh/pauli_tracker/status.svg)](https://deps.rs/repo/github/taeruh/pauli_tracker)

A library to track Pauli frames through a Clifford circuit with measurements.

## What is Pauli tracking

The Pauli group is invariant under the conjugation of Clifford operations. This means
that, in a circuit which consists only of Clifford operators and measurements, the Pauli
gates can be "pushed" to the end of the circuit, just before the measurements. The
benefit of this is that the Pauli gates don't have to be executed on a quantum computer
and can instead be accounted for by post-processing of the measurement outcomes. The
whole process is very similar to stabilzer simulations.

For more details about the Pauli and Clifford groups and Pauli tracking, please look
into the literature, e.g.,:
  - [Software Pauli Tracking for Quantum Computation](https://arxiv.org/abs/1401.5872v1)
  - [Stim: a fast stabilizer circuit simulator](https://arxiv.org/abs/2103.02202)
  - ...

[Conjugation rules](./docs/conjugation_rules.md) lists the conjugation rules for the
Clifford operations provided by the library (with proofs) (should be viewed with Katex).

## What this library does

This library does **not** provide a full quantum simulator. Instead it only provides
tools to track the Pauli operators through the circuit, while building up the circuit or
while simulating the circuit.

For examples, please look into the the [crate
documentation](https://docs.rs/pauli_tracker/#examples).

## License

Pauli Tracker is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
