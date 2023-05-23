// #![warn(missing_docs)] // turn on when things are more stable
#![deny(unsafe_op_in_unsafe_fn)]

use std::mem::ManuallyDrop;

use pauli_tracker::pauli_frame::{
    storage::MappedVector,
    Frames,
    Pauli,
    PauliStorage,
    PauliVec,
};

pub type Storage = MappedVector;
pub type Tracker = Frames<MappedVector>;

#[repr(C)]
pub struct RawStorage {
    frames: RawVec<PauliVec>,
    inverse_position: RawVec<usize>,
}

#[repr(C)]
pub struct RawVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

#[repr(C)]
pub struct RawPauliVec {
    left: RawVec<u32>,
    left_len: usize,
    right: RawVec<u32>,
    right_len: usize,
}

#[no_mangle]
pub extern "C" fn new_storage() -> *mut Storage {
    ManuallyDrop::new(Box::new(Storage::init(0))).as_mut() as *mut Storage
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn free_storage(storage: *mut Storage) {
    unsafe { Box::from_raw(storage) };
}

#[no_mangle]
pub extern "C" fn new_tracker() -> *mut Tracker {
    ManuallyDrop::new(Box::new(Tracker::init(0))).as_mut() as *mut Tracker
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn free_tracker(tracker: *mut Tracker) {
    unsafe { Box::from_raw(tracker) };
}

#[no_mangle]
pub extern "C" fn tracker_storage(tracker: &Tracker) -> *mut Storage {
    tracker.storage() as *const Storage as *mut Storage
}

#[no_mangle]
pub extern "C" fn raw_storage(storage: &mut Storage) -> RawStorage {
    let frames = storage.frames();
    let inverse_position = storage.inverse_position();
    RawStorage {
        frames: RawVec::<PauliVec> {
            ptr: frames.as_ptr() as *mut PauliVec,
            len: frames.len(),
            cap: frames.capacity(),
        },
        inverse_position: RawVec::<usize> {
            ptr: inverse_position.as_ptr() as *mut usize,
            len: inverse_position.len(),
            cap: inverse_position.capacity(),
        },
    }
}

#[no_mangle]
pub extern "C" fn track_x(tracker: &mut Tracker, qubit: usize) {
    tracker.track_pauli(qubit, unsafe { Pauli::from_unchecked(2) });
}

#[no_mangle]
pub extern "C" fn track_z(tracker: &mut Tracker, qubit: usize) {
    tracker.track_pauli(qubit, unsafe { Pauli::from_unchecked(1) });
}

#[no_mangle]
pub extern "C" fn track_y(tracker: &mut Tracker, qubit: usize) {
    tracker.track_pauli(qubit, unsafe { Pauli::from_unchecked(3) });
}

#[no_mangle]
pub extern "C" fn apply_h(tracker: &mut Tracker, qubit: usize) {
    tracker.h(qubit);
}

#[no_mangle]
pub extern "C" fn apply_s(tracker: &mut Tracker, qubit: usize) {
    tracker.s(qubit);
}

#[no_mangle]
pub extern "C" fn apply_cx(tracker: &mut Tracker, control: usize, target: usize) {
    tracker.cx(control, target);
}

#[no_mangle]
pub extern "C" fn apply_cz(tracker: &mut Tracker, qubit_a: usize, qubit_b: usize) {
    tracker.cx(qubit_a, qubit_b);
}

#[no_mangle]
pub extern "C" fn measure_and_store(
    tracker: &mut Tracker,
    qubit: usize,
    storage: &mut Storage,
) {
    tracker.measure_and_store(qubit, storage);
}

#[no_mangle]
pub extern "C" fn new_qubit(tracker: &mut Tracker, qubit: usize) {
    tracker.new_qubit(qubit);
}

#[repr(C)]
pub struct Tuple {
    qubit: usize,
    pauli: *const PauliVec,
}

#[no_mangle]
pub extern "C" fn sort_storage(storage: &Storage) -> RawVec<Tuple> {
    let mut ret = storage
        .iter()
        .map(|(i, p)| Tuple { qubit: i, pauli: p })
        .collect::<Vec<Tuple>>();
    ret.sort_by_key(|Tuple { qubit: i, .. }| *i);
    let mut ret = ManuallyDrop::new(ret);
    RawVec {
        ptr: ret.as_mut_ptr(),
        len: ret.len(),
        cap: ret.capacity(),
    }
}

#[no_mangle]
pub extern "C" fn free_sorted_storage(raw_vec: RawVec<Tuple>) {
    unsafe { Vec::from_raw_parts(raw_vec.ptr, raw_vec.len, raw_vec.cap) };
}

#[no_mangle]
pub extern "C" fn raw_pauli_vec(pauli_vec: &mut PauliVec) -> RawPauliVec {
    RawPauliVec {
        left: RawVec::<u32> {
            ptr: unsafe { pauli_vec.left.storage_mut() }.as_mut_ptr() as *mut u32,
            len: pauli_vec.left.storage().len(),
            cap: pauli_vec.left.capacity(),
        },
        left_len: pauli_vec.left.len(),
        right: RawVec::<u32> {
            ptr: unsafe { pauli_vec.right.storage_mut() }.as_mut_ptr() as *mut u32,
            len: pauli_vec.right.storage().len(),
            cap: pauli_vec.right.capacity(),
        },
        right_len: pauli_vec.right.len(),
    }
}
