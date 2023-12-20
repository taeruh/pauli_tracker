# Python wrapper around pauli_tracker

This Python package is a wrapper around [pauli_tracker
crate](https://github.com/taeruh/pauli_tracker/tree/main/pauli_tracker) exporting the
basic functionality.

When using this package, please also look at the
[documentation](https://docs.rs/pauli_tracker/latest/pauli_tracker/) of the Rust crate
(additionally to Python package's documentation) in parallel since it is much more
extensive about how the Pauli tracking works.

If some essential functionality is missing, because we just forgot to implement, please
open an issue or pull request (cf [contributing](https://github.com/taeruh/pauli_tracker/blob/main/CONTRIBUTING.md)). If you need more
functionality, it is fairly easy to use Rust from Python with the help of
[pyo3](https://github.com/PyO3/pyo3) and [maturin](https://github.com/PyO3/maturin).
However, because of [#1444](https://github.com/PyO3/pyo3/issues/1444), you probably want
to clone this repo and extend it.

## Examples

Please look at this [Python example](to_be_filled_in) and also at these [Rust
examples](https://docs.rs/pauli_tracker/latest/pauli_tracker/#examples). 

## Caution

Trying to build a Rust-Python package depending on this crate will probably not work
because of [#1444](https://github.com/PyO3/pyo3/issues/1444).

The API of the underling Rust crate is not stable (but the Python Package follows
SemVer).
