#!/usr/bin/env python


from pauli_tracker.live.map import Live
from pauli_tracker.frames.vec import Frames


def foo(a: int = 1) -> int:
    return 2 * a


if __name__ == "__main__":
    a = foo()
    print(a)

    tracker = Live()
    print(tracker.into_py_dict_recurse())
    tracker = Live(3)
    tracker.track_y(0)
    tracker.track_y(1)
    tracker.track_x(0)
    print(tracker.into_py_dict_recurse())
    a = tracker.get(0)
    print(a.tableau_encoding())
    a = tracker.get(1)
    print(a.tableau_encoding())

    tracker = Frames(3)
    tracker.track_y(0)
    tracker.track_y(1)
    tracker.track_x(0)
    a = tracker.get(0)
    print(tracker.into_py_array_recurse())  # bitvectors!
    print(a.sum_up([True] * 3).into_py_tuple())
