#!/usr/bin/env python


from pauli_tracker.frames.map import Frames
from pauli_tracker.pauli import PauliStack


if __name__ == "__main__":
    frames = Frames.deserialize("output/hey.json")
    serialized = frames.serialize_to_string()
    # print(serialized)
    frames = Frames.deserialize_from_string(serialized)
    frames.serialize("output/hey.bin", serialization_format="bincode")
    frames = Frames.deserialize("output/hey.bin", serialization_format="bincode")
    print(frames.into_py_dict_recursive())
