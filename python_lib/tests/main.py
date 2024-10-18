#!/usr/bin/env python


from pauli_tracker.frames.map import Frames

if __name__ == "__main__":
    frames = Frames(3)
    frames.track_x(1)
    frames.track_x(1)
    map = [0, 0]
    graph = frames.get_order(map)
    print(graph.take_into_py_graph())
