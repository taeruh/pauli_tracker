"""
Pauli encoding.

This module is a very simplified version of the `original pauli module
<https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/index.html>`_ at the moment.
"""

from pauli_tracker._lib.pauli import PauliDense, PauliTuple, PauliStack


class TableauEncoding:
    """
    `tableau_encoding
    <https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/tableau_encoding/index.html>`_
    """

    I = 0
    X = 2
    Y = 3
    Z = 1
