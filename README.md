# Pauli Tracker

[![Crates.io](https://img.shields.io/crates/v/pauli_tracker.svg)](https://crates.io/crates/pauli_tracker)
[![Documentation](https://docs.rs/pauli_tracker/badge.svg)](https://docs.rs/pauli_tracker/)
[![Codecov](https://codecov.io/github/taeruh/pauli_tracker/coverage.svg?branch=main)](https://codecov.io/gh/taeruh/pauli_tracker)
[![Dependency status](https://deps.rs/repo/github/taeruh/pauli_tracker/status.svg)](https://deps.rs/repo/github/taeruh/pauli_tracker)
(Rust crate)\
[![PyPI.org](https://img.shields.io/pypi/v/pauli-tracker.svg)](https://pypi.org/project/pauli-tracker/)
[docs](https://taeruh.github.io/pauli_tracker/)
(Python package)

A library to track Pauli gates through a Clifford circuit.

## What is Pauli tracking

The Pauli group is invariant under the conjugation of Clifford operations. This means
that, in a circuit which consists only of Clifford operators and measurements, the Pauli
gates can be "pushed" to the end of the circuit, just before the measurements. The
benefit of this is that the Pauli gates don't have to be executed on a quantum computer
and can instead be accounted for by post-processing of the measurement outcomes. The
whole process is very similar to stabilizer simulations. Additionally, the information
captured by doing the Pauli tracking during compilation can be used for certain
optimizations in measurement-based quantum computing (MBQC).

For more details about the Pauli and Clifford groups and Pauli tracking, please look
into the literature, e.g.,:
  - standard quantum textbooks
  - [Software Pauli Tracking for Quantum Computation] 
  - [Stim: a fast stabilizer circuit simulator]
  - ...

[Conjugation rules] lists the conjugation rules for the Clifford operations provided by
the library (with proofs) and also contains some useful theory related to Pauli
tracking.

## What this library does

This library does **not** provide a quantum simulator. Instead it only provides tools to
track the Pauli operators through the circuit, while compiling the circuit or while
simulating the circuit.

The project is foremost a Rust library providing the [pauli_tracker crate]. It's a
generic and hopefully flexible library which allows you to choose different core data
structures fitted to the problem you want to solve.

However, we also have [Python package] that wraps the basic functionality for easy use
from Python. Note that this wrapper might randomly miss some important functionality
because we just forgot to wrap it. In that case, please open an issue or a pull request.

## Examples

For example code, please look into the [crate documentation] and maybe also at the
[Python documentation].

## Related projects

- [mbqc_scheduling](https://github.com/taeruh/mbqc_scheduling): Using the Pauli tracking
information to solve a scheduling problem in measurement-based quantum computing, that
is, when to initialize and when to measure which qubit for space-time optimality.

## Issues and Contributing

Don't hold back with issues, I need some feedback at the current stage. Contributions
are very welcome, e.g., if you find bugs or need additional Clifford gates. Check out
the [contributing guidelines].

## How to cite

When you use the Pauli Tracker for your research please cite it (currently just by
linking to this repo; a paper is in progress).

## License

The Pauli Tracker project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

[Conjugation rules]: https://github.com/taeruh/pauli_tracker/blob/main/docs/conjugation_rules.pdf
[contributing guidelines]: https://github.com/taeruh/pauli_tracker/blob/main/CONTRIBUTING.md
[crate documentation]: https://docs.rs/pauli_tracker/#examples
[pauli_tracker crate]: https://github.com/taeruh/pauli_tracker/blob/main/pauli_tracker
[Python documentation]: https://taeruh.github.io/pauli_tracker/
[Python package]: https://github.com/taeruh/pauli_tracker/tree/main/python_lib#readme
[Software Pauli Tracking for Quantum Computation]: https://arxiv.org/abs/1401.5872v1
[Stim: a fast stabilizer circuit simulator]: https://arxiv.org/abs/2103.02202
