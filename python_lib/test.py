#!/usr/bin/env python


from pauli_tracker.live.map import Live
from pauli_tracker.frames.vec import Frames
from pauli_tracker.pauli import PauliDense


if __name__ == "__main__":
    tracker = Live(3)
    tracker.track_y(0)
    tracker.track_y(1)
    tracker.track_x(0)
    print(tracker.to_py_dict())
    a = PauliDense(tracker.get(0))
    a.show()
    a.pauli = tracker.get(1)
    a.show()

    tracker = Frames(3)
    tracker.track_y(0)
    tracker.track_y(1)
    tracker.track_x(0)
    print(tracker.to_py_array())  # bitvectors!
