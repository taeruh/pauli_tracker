# Python wrapper around pauli_tracker

This Python package is a wrapper around [pauli_tracker crate] exporting the basic
functionality.

When using this package, please also look at the [crate's documentation] of the Rust
crate (additionally to [Python package's documentation] of course) in parallel since it is
much more extensive about how the Pauli tracking works.

If some essential functionality is missing, because we just forgot to implement it, please
open an issue or pull request (cf [contributing]). If you need more functionality, it is
fairly easy to use Rust from Python with the help of [pyo3] and [maturin]. However,
because of [#1444], you probably want to clone this repo and extend it.

## Examples

Please look at this [Python example] and also at these [Rust examples].

## Installation

Until the wrapper is released as a package on PyPI, you can download it from the artifacts
of the Github actions that have "pypackage" as workflow, e.g., from the [latest build].
Just choose the right build for your OS and Python version (for Linux, the builds for the
different Python versions are all bundled in the "linux-wheels" artifact; they are all
build for manylinux\_2\_28\_x86\_64, cf [manylinux]; they also contain an abi3 build for
Python>=3.8). You may have to unzip the artifact. Then you can install the package with
`pip install <path-to-whl-file>`.

## Manually Building

The package has to be build with [maturin]. The `make package` commands builds it through
a docker container such that it is compatible with manylinux\_2\_28\_x86\_64 for Python >=
3.8. With `make update_docs` the documentation can be build. The output of both make
commands is in the `dist` directory.

## Caution

Trying to build a Rust-Python package depending on this the underlying Rust create here
will probably not work because of [#1444].

The API of the underling Rust crate is not stable (but the Python package follows
SemVer).

[crate's documentation]: https://docs.rs/pauli_tracker/latest/pauli_tracker/
[contributing]: https://github.com/taeruh/pauli_tracker/blob/main/CONTRIBUTING.md
[latest build]: https://github.com/taeruh/pauli_tracker/actions/runs/7538212166
[manilinux]: https://github.com/pypa/manylinux
[maturin]: https://github.com/PyO3/maturin
[pauli_tracker crate]: https://github.com/taeruh/pauli_tracker/tree/main/pauli_tracker
[pyo3]: https://github.com/PyO3/pyo3
[Python example]: https://taeruh.github.io/pauli_tracker/#example-usage
[Python package's documentation]: https://taeruh.github.io/pauli_tracker/
[Rust examples]: https://docs.rs/pauli_tracker/latest/pauli_tracker/#examples
[#1444]: https://github.com/PyO3/pyo3/issues/1444
