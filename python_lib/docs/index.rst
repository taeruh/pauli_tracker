Welcome to pauli_tracker's documentation!
=========================================

**This library is a Python wrapper around the** `pauli_tracker crate`_ **(Rust
library).**

Only the essential functionality is exposed in this wrapper, but for most use cases,
this should be sufficient.

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

**Example usage**

.. code-block:: python

   from pauli_tracker.live.map import Live

   # Pauli encoding: 0 -> I, 1 -> Z, 2 -> X, 3 -> Y

   tracker = Live(3)  # initialize the tracker with 3 qubits
   tracker.track_x(0)  # track an X Pauli on qubit 0
   tracker.track_y(1)  # track an Y Pauli on qubit 0
   tracker.h(0)  # apply a Hadamard gate on qubit 0
   tracker.cx(1, 2)  # apply a CNOT gate on control qubit 1 and target qubit 2
   print(tracker.measure(0).tableau_encoding())  # measure qubit 0
   tracker.new_qubit(4)  # add a new qubit at label 4
   # transform the whole opaque type into a standard Python type
   print(tracker.into_py_dict_recursive())

For more examples of how the Pauli tracking works, please take a look at `Rust example
code`_ (although not everything is exposed in this wrapper, but it should still give a
good idea about how things work).

**Caution**

For all classes we define the *magic __new__* method and not the *magic __init__* method
(that's what `pyo3`_ does). However, we still define a *normal __init__* method for
documentation purposes (docstrings on __new__ doesn't seem to work properly ...?). This
method does nothing. It is not called when constructing objects with the standard
constructor syntax. However, it is called when explictily calling *__init__*, e.g., when
doing something like `super().__init__()` in a subclass' *magic init*. In that case, the
*magic __init__* method is not called, I think (but *magic __new__* is still called in
the subclass' *magic __init__*). But all this shouldn't matter, I hope, since both,
*magic __init__* and *normal __init__* do nothing.

.. _pauli_tracker crate:
   https://docs.rs/pauli_tracker/latest/pauli_tracker
.. _Rust example code:
   https://docs.rs/pauli_tracker/latest/pauli_tracker/#examples
.. _pyo3:
   https://github.com/PyO3/pyo3
.. _GitHub repository:
   https://github.com/taeruh/pauli_tracker
.. _Tracker:
   https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/trait.Tracker.html
.. _conversion rules:
   https://pyo3.rs/v0.20.0/conversions/tables

.. autosummary::
   :toctree: _autosummary
   :template: custom_module.rst
   :recursive:

   pauli_tracker

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
