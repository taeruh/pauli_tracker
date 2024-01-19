#!/usr/bin/env python


from pauli_tracker.frames.map import Frames
from pauli_tracker.pauli import PauliStack


if __name__ == "__main__":
    tracker = Frames(2)
    tracker.track_x(0)
    tracker.track_y(1)

    transposed = tracker.stacked_transpose_reverted(2)
    print(transposed.into_py_matrix())

    frame = transposed.pop()
    print(frame.into_py_tuple())
    print(frame.get(1).into_py_tuple())
    frame.xor_inplace(transposed.pop())
    print(frame.into_py_tuple())
    print(transposed.pop())

