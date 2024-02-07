# Python wrapper around pauli_tracker

This Python package is a wrapper around the [pauli_tracker crate] exporting the basic
functionality.

When using this package, please also look at the [crate's documentation] of the Rust
crate (additionally to the [Python package's documentation]) in parallel since it is much
more extensive about how the Pauli tracking works (although not everything there is
supported in this wrapper).

If some essential functionality is missing, because we just forgot to implement it, please
open an issue or pull request (cf. [contributing]). If you need more functionality, it is
fairly easy to use Rust from Python with the help of [pyo3] and [maturin]. However,
because of [#1444], you may want to clone this repository and extend it.

## Examples

Please look at this [Python example] and also at these [Rust examples].

## Installation

You can install the package from PyPI, e.g., with
```bash
pip install pauli-tracker
```
The package contains pre-built wheels for manylinux\_2\_28\_x86\_64 (works on most Linux
distribuitions), latest Windows and latest MacOS (latest with respect to when the package
was built) for Python 3.8 to 3.12. Additionally, there is an manylinux\_2\_28\_x86\_64
abi3 wheel for Python >= 3.8. You can also build the package from source, e.g., force it
during a pip install with `pip install --no-binary pauli-tracker pauli-tracker`, however,
note that this requires Python >= 3.8 and a Rust toolchain >= 1.65.

At the moment, you may also find a more up-to-date wheel in the artifacts of the latest
"pypackage" github actions workflow, this is unstable though.

### Manually Building

The package has to be build with [maturin]. The `make package` commands builds it through
a docker container such that it is compatible with manylinux\_2\_28\_x86\_64 for Python >=
3.8. With `make update_docs` the documentation can be build. The output of both make
commands is in the `dist` directory.

## Versioning

The Python package follows SemVer, however, the underlying Rust crate is unstable.

[crate's documentation]: https://docs.rs/pauli_tracker/latest/pauli_tracker/
[contributing]: https://github.com/taeruh/pauli_tracker/blob/main/CONTRIBUTING.md
[manylinux]: https://github.com/pypa/manylinux
[maturin]: https://github.com/PyO3/maturin
[pauli_tracker crate]: https://github.com/taeruh/pauli_tracker/tree/main/pauli_tracker
[pyo3]: https://github.com/PyO3/pyo3
[Python example]: https://taeruh.github.io/pauli_tracker/#example-usage
[Python package's documentation]: https://taeruh.github.io/pauli_tracker/
[Rust examples]: https://docs.rs/pauli_tracker/latest/pauli_tracker/#examples
[#1444]: https://github.com/PyO3/pyo3/issues/1444
