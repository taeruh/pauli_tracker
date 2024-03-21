#!/usr/bin/env python


from pauli_tracker.frames.map import Frames
from pauli_tracker.pauli import PauliStack


if __name__ == "__main__":
    frames = Frames(3)
    frames.track_x(0)
    frames.track_z(1)
    frames.track_y(2)
    frames.cy(0, 2)
    frames.serialize("output/hey.json")
    print(frames.into_py_dict_recursive())
    frames.remove_x(0)
    frames.zcx(1, 2)
    print(frames.into_py_dict_recursive())
    frames = Frames.deserialize("output/hey.json")
    serialized = frames.serialize_to_string()
    # print(serialized)
    frames = Frames.deserialize_from_string(serialized)
    frames.serialize("output/hey.bin", serialization_format="bincode")
    frames = Frames.deserialize("output/hey.bin", serialization_format="bincode")
    print(frames.into_py_dict_recursive())
