"""
Pauli encoding (, transformation and arithmetic).

This module is a very simplified version of the `original pauli module
<https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/index.html>`_. At the moment,
it is not really useful and only describes how integers (:obj:`PauliDense`) and tuples
(:obj:`PauliTuple`) are encoded.
"""

import abc


class Pauli(abc.ABC):
    """Abstract base class describing methods that are supported by Pauli's"""

    @abc.abstractmethod
    def show(self):
        """Show how the Pauli is decoded."""
        pass


class PauliDense(Pauli):
    """Pauli encoded as an integer."""

    def __init__(self, pauli: int):
        """
        Args:
            pauli: 0 <-> I, 1 <-> Z, 2 <-> X, 3 <-> Y
        """
        if not 0 <= pauli <= 3:
            raise ValueError("pauli must be in [0, 3]")
        self.pauli = pauli

    def show(self):
        match self.pauli:
            case 0:
                print("I", self.pauli)
            case 1:
                print("Z", self.pauli)
            case 2:
                print("X", self.pauli)
            case 3:
                print("Y", self.pauli)


class PauliTuple(Pauli):
    """Pauli encoded into two booleans."""

    def __init__(self, pauli: tuple[bool, bool]):
        """
        Args:
            pauli: the first element is the X component, the second the Z component
        """
        self.pauli = pauli

    def show(self):
        match self.pauli:
            case (False, False):
                print("I", self.pauli)
            case (False, True):
                print("Z", self.pauli)
            case (True, False):
                print("X", self.pauli)
            case (True, True):
                print("Y", self.pauli)
