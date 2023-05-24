lib = "../../../target/debug/libc_pauli_tracker.so"

struct Storage end
struct PauliVec end
struct Tracker end

struct RawVec{T}
    ptr::Ptr{T}
    len::UInt
    cap::UInt
end

struct RawPauliVec
    left::RawVec{UInt32}
    left_len::UInt32
    right::RawVec{UInt32}
    right_len::UInt32
end

struct RawStorage
    frames::RawVec{PauliVec}
    inverse_position::RawVec{UInt}
end


struct Tuple
    qubit::UInt
    pauli::Ptr{PauliVec}
end

function new_tracker()::Ptr{Tracker}
    @ccall lib.new_tracker()::Ptr{Tracker}
end

function free_tracker(tracker::Ptr{Tracker})
    @ccall lib.free_tracker(tracker::Ptr{Tracker})::Cvoid
end

function new_storage()::Ptr{Storage}
    @ccall lib.new_storage()::Ptr{Storage}
end

function free_storage(storage::Ptr{Storage})
    @ccall lib.free_storage(storage::Ptr{Storage})::Cvoid
end


function tracker_storage(tracker::Ptr{Tracker})::Ptr{Storage}
    @ccall lib.tracker_storage(tracker::Ptr{Tracker})::Ptr{Storage}
end

function track_x(tracker::Ptr{Tracker}, qubit::UInt)
    @ccall lib.track_x(tracker::Ptr{Tracker}, qubit::UInt)::Cvoid
end


function track_z(tracker::Ptr{Tracker}, qubit::UInt)
    @ccall lib.track_z(tracker::Ptr{Tracker}, qubit::UInt)::Cvoid
end


function track_y(tracker::Ptr{Tracker}, qubit::UInt)
    @ccall lib.track_y(tracker::Ptr{Tracker}, qubit::UInt)::Cvoid
end


function apply_h(tracker::Ptr{Tracker}, qubit::UInt)
    @ccall lib.apply_h(tracker::Ptr{Tracker}, qubit::UInt)::Cvoid
end


function apply_s(tracker::Ptr{Tracker}, qubit::UInt)
    @ccall lib.apply_s(tracker::Ptr{Tracker}, qubit::UInt)::Cvoid
end


function apply_cx(tracker::Ptr{Tracker}, control::UInt, target::UInt)
    @ccall lib.apply_cx(tracker::Ptr{Tracker}, control::UInt, target::UInt)::Cvoid
end


function apply_cz(tracker::Ptr{Tracker}, qubit_a::UInt, qubit_b::UInt)
    @ccall lib.apply_cx(tracker::Ptr{Tracker}, qubit_a::UInt, qubit_b::UInt)::Cvoid
end

function measure_and_store(tracker::Ptr{Tracker}, qubit::UInt, storage::Ptr{Storage})
    @ccall lib.measure_and_store(
        tracker::Ptr{Tracker},
        qubit::UInt,
        storage::UInt
    )::Cvoid
end

function new_qubit(tracker::Ptr{Tracker}, qubit::UInt)
    @ccall lib.new_qubit(tracker::Ptr{Tracker}, qubit::UInt)::Cvoid
end

function sort_storage(storage::Ptr{Storage})::RawVec{Tuple}
    @ccall lib.sort_storage(storage::Ptr{Storage})::RawVec{Tuple}
end

function free_sorted_storage(raw_vec::RawVec{Tuple})
    @ccall lib.free_sorted_storage(raw_vec::RawVec{Tuple})::Cvoid
end

function raw_pauli_vec(pauli_vec::Ptr{PauliVec})::RawPauliVec
    @ccall lib.raw_pauli_vec(pauli_vec::Ptr{PauliVec})::RawPauliVec
end
