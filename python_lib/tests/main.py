#!/usr/bin/env python


from pauli_tracker.live.map import Live
from pauli_tracker.frames.vec import Frames


def foo(a: int = 1) -> int:
    return 2 * a


if __name__ == "__main__":
    a = foo()
    print(a)

    tracker = Live()
    print(tracker.into_py_dict_recursive())
    tracker = Live(3)
    tracker.track_y(0)
    tracker.track_y(1)
    tracker.track_x(0)
    print(tracker.into_py_dict_recursive())
    a = tracker.get(0)
    print(a.tableau_encoding())
    a = tracker.get(1)
    print(a.tableau_encoding())

    # print(tracker.serialize.__doc__)
    # print(tracker.deserialize.__doc__)

    tracker.serialize("output/foo.json", "bincode")

    tracker = Frames(3)
    tracker.track_y(1)
    tracker.track_y(2)
    a = tracker.get(0)
    print(tracker.into_py_array_recursive())  # bitvectors!
    print(a.sum_up([True] * 3).into_py_tuple())
    tracker.serialize("output/bar.json", "bincode")
    new_tracker = Frames.deserialize("output/bar.json", "bincode")
    # print(new_tracker.into_py_array_recursive())

    dep_graph = tracker.create_dependency_graph([0, 1])
    # dep_graph.serialize("output/bar.json")
    dep_graph = dep_graph.into_py_graph()
    print(dep_graph)
    print(tracker.create_py_dependency_graph([0, 1]))
