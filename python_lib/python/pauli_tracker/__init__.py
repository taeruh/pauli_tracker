"""
Wrapper around the essential functionality of the `pauli_tracker crate`_.

When exporting Rust code through the FFI, we loose the ability to be generic. Because of
that we can only support specific types in this wrapper. The submodule structure kinda
emulates these types. For example, the :obj:`Live <.live.map.Live>` in :mod:`.live.map`
corresponds to Rust's `Live`_\\<`Map`_\\<_>> type.

.. _pauli_tracker crate:
   https://docs.rs/pauli_tracker/latest/pauli_tracker
.. _Live:
   https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/live/struct.Live.html
.. _Map:
   https://docs.rs/pauli_tracker/latest/pauli_tracker/collection/type.Map.html
"""


def bitvector_to_boolvector(bitvec: list[int], length: int) -> list[bool]:
    """Convert a bitvector to a boolvector.

    The bitvector is a list of 64 bit chunks, and in each chunk the bits are ordered from
    least significant to most significant.

    The implementation is very simplistic; you may want to use another more efficient
    method to access the bits.

    Args:
        bitvec: The bitvector to convert.
        length: The total number of bits.

    Returns:
        The boolvector.
    """

    def update(ret, bits):
        for bit in reversed(bits[2:]):  # skip 0b prefix
            if bit == "0":
                ret.append(False)
            else:
                ret.append(True)

    ret = []
    for chunk in bitvec[:-1]:
        update(ret, format(chunk, "#066b"))  # +2 padding because of 0b prefix
    update(ret, format(bitvec[-1], f"#0{length - (len(bitvec) - 1) * 64 + 2}b"))
    return ret
