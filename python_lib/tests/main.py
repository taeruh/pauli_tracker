#!/usr/bin/env python


from pauli_tracker import frames
from pauli_tracker.frames.map import Frames
from pauli_tracker.pauli import PauliStack


if __name__ == "__main__":
    tracker = Frames(1)

    length = 1
    tracker.track_y(0)
    for i in range(64):
        length += 1
        tracker.track_x(0)
    for i in range(64):
        length += 1
        tracker.track_z(0)
    for i in range(64):
        length += 1
        tracker.track_y(0)

    stack = tracker.into_py_dict_recursive()[0]
    bool_stack = (
        frames.bitvector_to_boolvector(stack[0], length),
        frames.bitvector_to_boolvector(stack[1], length),
    )
    print(bool_stack)
    print(stack)
    print(length, (len(bool_stack[0]), len(bool_stack[1])))

    a = PauliStack()
