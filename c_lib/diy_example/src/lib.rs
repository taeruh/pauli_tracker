use pauli_tracker::{
    collection::{Init, Map},
    pauli::{Pauli, PauliDense},
    tracker::{live, Tracker},
};

// define the specific types that you need
type Live = live::Live<Map<PauliDense>>;

#[no_mangle]
pub extern "C" fn create(size: usize) -> *mut Live {
    Box::into_raw(Box::new(Live::init(size)))
}

// manually "freeing" the memory
#[no_mangle]
extern "C" fn drop(ptr: *mut Live) {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
extern "C" fn track_x(live: &mut Live, qubit: u32) {
    live.track_x(qubit as usize);
}

#[no_mangle]
extern "C" fn get(live: &Live, qubit: usize) -> u8 {
    live.get(qubit).expect("invalid qubit").tableau_encoding()
}

// extend with further functionality that you need
