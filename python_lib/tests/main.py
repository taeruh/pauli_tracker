#!/usr/bin/env python


from pauli_tracker.frames.map import Frames
from pauli_tracker.pauli import PauliStack


if __name__ == "__main__":
    frames = Frames.deserialize("output/heyold.json")
    print(frames.into_py_dict_recursive())
