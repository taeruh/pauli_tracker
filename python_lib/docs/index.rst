.. pauli_tracker documentation master file, created by
   sphinx-quickstart on Sat Dec 16 12:37:50 2023.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to pauli_tracker's documentation!
=========================================

**This library is a Python wrapper around the** `pauli_tracker crate`_ **(Rust
library).**

Only the essential functionality is exposed in this wrapper, but for most use cases,
this should be sufficient. If you need more functionality, it is fairly easy to
use Rust from Python with the help of `pyo3`_ and `maturin`_.

*If you think something should be included in this wrapper here, please open an
issue or a pull request on the* `GitHub repository`_.*

**How to read this documentation**

Since this is just a wrapper the documentation is very sparse and we mostly refer to
the documentation of the `pauli_tracker crate`_. It is recommended to have at least
a look at the top-level documentation of the `pauli_tracker crate`_ to get an idea
of what this library does.

Most methods have the identical name as their counterparts in the `pauli_tracker
crate`_ (specifically, in the `Tracker`_ trait). If these methods do not have a
docstring, they act exactly the same and you can find the documentation in the
`pauli_tracker crate`_ (use the search bar to quickly find them).

Typing annotations and IDE support are completely missing at the moment. If the
types are not documented, you can get them from the according documenation in the
`pauli_tracker crate`_, converting with the help of these `conversion rules`_. For
example, a return value of `Result<T, E>` in Rust becomes `T` on succes and raises
an exception describing `E` on failure in Python.

*When the `pauli_tracker crate`_ reaches a stable version, we will add proper
documentation here (stub files maybe sooner when pyo3 can generate them
automatically).*

.. _pauli_tracker crate:
   https://docs.rs/pauli_tracker/latest/pauli_tracker
.. _pyo3:
   https://github.com/PyO3/pyo3
.. _maturin:
   https://github.com/PyO3/maturin
.. _GitHub repository:
   https://github.com/taeruh/pauli_tracker
.. _Tracker:
   https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/trait.Tracker.html
.. _conversion rules:
   https://pyo3.rs/v0.20.0/conversions/tables

.. autosummary::
   :toctree: _autosummary
   :template: custom_module_template.rst
   :recursive:

   pauli_tracker

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
