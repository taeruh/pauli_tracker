#!/usr/bin/env python

import pauli_tracker

if __name__ == "__main__":
    tracker = pauli_tracker.Frames(3)
    tracker.track_y(0)
    tracker.track_y(1)
    tracker.track_y(0)
    print(tracker.to_py_dict())
