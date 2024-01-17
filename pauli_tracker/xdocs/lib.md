A library to track Pauli frames through a Clifford circuit with measurements. A
general brief introduction to Pauli tracking is given in the repository's
[README].

# Crate features

* **serde**
  Support [serde] the main data types.
* **circuit**
  Includes the [circuit] module which contains tools to combine the Pauli tracking
  mechanism with a circuit simulator/description.
* **bitvec**
  Implement [BooleanVector] for [bitvec::vec::BitVec] (extern crate). Note that we do
  not export any types of [bitvec]; you need to depend on it manually to use its types.
* **bit-vec**
  Implement [BooleanVector] for [bit_vec::BitVec] (extern crate). Note that we do not
  export any types of [bit-vec]; you need to depend on it manually to use its types.
* **bitvec_simd**
  Implement [BooleanVector] for wrapper [SimdBitVec] around [bitvec_simd::BitVec]
  (extern crate). Note that while this bit-vector implementation uses SIMD operations
  (if available), it also uses the crate [smallvec] for its inner storage. That may be
  not memory efficient for the Pauli tracking since the storage is fairly big.

# Examples

### A first idea

This examples gives a first introduction to the tracking mechanism. The example
requires the [rand] crate.
```rust
# #[cfg_attr(coverage_nightly, coverage(off))]
# // "circuit" instead of "rand" because we do not export the "rand" feature, since we
# // use it as dep:rand
# #[cfg(feature = "circuit")] // we need it to activate the "rand" dep
# fn main() {
# #[rustfmt::skip]
use pauli_tracker::{
    collection::{Map, BufferedVector, Full, Init},
    pauli::{self, Pauli, PauliTuple},
    tracker::{Tracker, live, frames},
};
// first, we will use the Frames tracker to fully track all Pauli frames

// some types in this library are pretty generic; it's almost always a good idea to
// define some type aliases
type BoolVec = Vec<bool>; // you might want to use a "bit-vector"; cf. features
type PauliStack = pauli::PauliStack<BoolVec>;
type Storage = Map<PauliStack>;
type Frames = frames::Frames<Storage>;
type Live = live::Live<BufferedVector<PauliTuple>>;

// initialize the tracker with three qubits
let mut tracker = Frames::init(3);

// track Paulis through an (imaginary) circuit
// X(0), CX(0, 1), S(1), Z(2), CZ(1, 2), H(0)
tracker.track_x(0); // track a new Pauli X; frame (0)
tracker.cx(0, 1); // conjugate with a Control X gate
tracker.s(1); // conjugate with an S gate
tracker.track_y(2); // track a new Pauli Z; frame (1)
tracker.cz(1, 2); // conjugate with a Control Z gate
tracker.h(0); // conjugate with an H gate

// let's get the frames (sorted into a Vec for convenience)
let frames = tracker.clone().into_storage().into_sorted_by_key();

// what would we expect (calculate it by hand)?
let mut expected =
    vec![(0, PauliStack::new()), (1, PauliStack::new()), (2, PauliStack::new())];
// {{ frame (0)
expected[0].1.push(PauliTuple::new_z());
expected[1].1.push(PauliTuple::new_y());
expected[2].1.push(PauliTuple::new_z());
// }}
// {{ frame (1)
expected[0].1.push(PauliTuple::new_i());
expected[1].1.push(PauliTuple::new_z());
expected[2].1.push(PauliTuple::new_y());
// }}
// (creating `expected` can be faster achieved with PauliStack::try_from_str, e.g., as in
// the example "The dependency graph")

// let's check it
assert_eq!(frames, expected);

// Note that the frames-matrix has the qubits on the major axis and the frames on the
// minor axis, for performance reasons while tracking. When the tracking is done, one
// can tranpose the matrix, which is beneficial for certain applications, e.g., when
// iteratively adding up the frames.
assert_eq!(
    tracker.transpose_reverted::<PauliTuple>(3) // we need to pass in the number of qubits
        .into_iter().map(|frame| PauliStack::from_iter(frame)).collect::<Vec<_>>(),
    vec![ //             qubit  X 012  Z 012  (tableau representation)
        PauliStack::try_from_str("001", "011").unwrap(), // frame (1)
        PauliStack::try_from_str("010", "111").unwrap(), // frame (0)
    ]
);
// Note that the frames order is reverted, so that one can get the frames in order when
// popping from the vector

// Let's vary the example from above a little bit: Paulis are often induced as
// corrections in MBQC. These corrections might effect the measurement basis of
// following measurements. To get the final correction before a measurement we could add
// the frames in `frames` from above, however, we can also do it directly with the
// LiveTracker:

let mut tracker = Live::init(3); // initialize the tracker with three qubits

// a small helper to track Paulis conditioned on measurements (the circuit module
// provides similar helpers)
let mut measurements = Vec::<bool>::new();
let mut correct = |tracker: &mut Live, bit, pauli| {
    // "measurement"; in a real use case this would be, for example, a quantum
    // measurement
    let outcome = rand::random::<bool>();
    if outcome {
        tracker.track_pauli(bit, pauli);
    }
    measurements.push(outcome);
};

// the same circuit from above, but with conditional Paulis, e.g., MBQC corrections
correct(&mut tracker, 0, Pauli::new_x());
tracker.cx(0, 1);
tracker.s(1);
correct(&mut tracker, 2, Pauli::new_y());
tracker.cz(1, 2);
tracker.h(0);

// let's checkout the final corrections
println!("{tracker:?}");

// we can check whether the output of the live tracker aligns with the frames
// tracker (in a real application one would probably do this by iteratively adding up
// the frames with the help of Frames::transposed_reverted)
let conditional_summed_frames: Vec<_> = frames
    .into_iter()
    .map(|(_, pauli_stack)| pauli_stack.sum_up(&measurements))
    .collect();
assert_eq!(tracker.as_ref().0, conditional_summed_frames, "{measurements:?}");
# }
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(not(feature = "circuit"))]
# fn main() {}
```

### The dependency graph
This example introduces the [create_dependency_graph] function that can be used to
analyse measurement dependencies.
```rust
# #[cfg_attr(coverage_nightly, coverage(off))]
# fn main() {
# #[rustfmt::skip]
use pauli_tracker::{
    collection::{BufferedVector, Iterable, Init},
    pauli::{self, Pauli},
    tracker::{Tracker, frames::{self, dependency_graph}},
};
type BoolVec = Vec<bool>;
type PauliStack = pauli::PauliStack<BoolVec>;
// we want a fixed order in our storage for this test, so we use BufferedVector and not
// Map
type Storage = BufferedVector<PauliStack>;
type Frames = frames::Frames<Storage>;

// let's consider the following tracking pattern
let mut tracker = Frames::init(6);

tracker.track_x(0); // frame (0)
tracker.cx(0, 1);
tracker.s(1);
tracker.track_y(2); // frame (1)
tracker.cz(1, 2);
tracker.cx(3, 2);
tracker.track_z(1); // frame (2)
tracker.h(1);
tracker.cx(3, 1);
tracker.cz(3, 2);

// check its output
assert_eq!(
    tracker.as_storage().sort_by_key(),
    vec![
        // tableau representation:      X      Z   (the columns are the frames)
        (0, &PauliStack::try_from_str("100", "000").unwrap()),
        (1, &PauliStack::try_from_str("111", "100").unwrap()),
        (2, &PauliStack::try_from_str("010", "110").unwrap()),
        (3, &PauliStack::try_from_str("000", "000").unwrap()),
        (4, &PauliStack::try_from_str("000", "000").unwrap()),
        (5, &PauliStack::try_from_str("000", "000").unwrap()),
    ]
);

// now, we assume that when the real circuit is executed, the paulis are conditionally
// tracked on measurement outcomes of the qubits 3,4 and 0, i.e.
let map = [
    // describe the relation between frames and qubits
    4, // frame (0) depends on the measurement on qubit 4
    5, // frame (1) on 5
    0, // frame (2) on 0
];

// we are interested in how many steps of parallel measurement we need to measure qubits
// "0" to "4"; this can be figured out with the dependency graph:
let graph = dependency_graph::create_dependency_graph(
    Iterable::iter_pairs(tracker.as_storage()), &map);

// in this case the graph is already sorted according to the node numbers, but that is
// not always true, if not one can use storage::sort_layers_by_key to sort it, if
// needed

assert_eq!(
    graph, // fixed order because we set Storage = Vector
    // the tuples consist of the qubit and its measurement dependencies
    vec![
        vec![(3, vec![]), (4, vec![]), (5, vec![])], // layer 0
        vec![(0, vec![4]), (2, vec![4, 5])],         // layer 1
        vec![(1, vec![5, 0])],                       // layer 2
    ]
);
# }
// - in layer 0, there are no Paulis before the measurements, i.e., we have no
//   dependecies; the qubits in layer 1 depend only on outcomes of qubits in layer 0;
//   the qubits in layer 2 depend only on qubits in layer 0, ..., 1; and so on
// - the graph removes redundent dependencies, e.g., although qubit 1 depends on
//   [0, 4, 5] (cf. the output of the tracker above), the graph only lists [0, 5]; this
//   is because qubit 0 already depends on the outcome of qubit 4
// - we see that the graph has three layers, this means that the six measurements
//   can be performed in 3 steps
```

### Streamed tracking
This example focuses on how to stream parts Pauli tracker's storage, which might be
usefull if the circuit is really large and one runs into memory problems. As example
circuit we choose the Toffoli gate decomposed into Clifford + (teleported) T gates.
Check out this [paper] (specifically Fig. (6) and Fig. (12)) to see how the Toffoli gate
can be decomposed into Clifford + T gates and how T gates can be teleported.

We use the [circuit] module and [bit_vec::BitVec], i.e., the example requires the
features "circuit" and "bit-vec", as well as a dependency on the bit_vec crate.
```rust
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(all(feature = "circuit", feature = "bit-vec"))]
# fn main() {
# #[rustfmt::skip]
use pauli_tracker::{
    circuit::{CliffordCircuit, DummyCircuit, TrackedCircuit},
    collection::{Map, BufferedVector, Iterable, Full, Init},
    pauli::{self, Pauli},
    tracker::{Tracker, frames::{self, dependency_graph}},
};

type BoolVec = bit_vec::BitVec;
type PauliStack = pauli::PauliStack<BoolVec>;
type Storage = Map<PauliStack>;
type Frames = frames::Frames<Storage>;

// a wrapper around (pseude) circuit (simulator), a tracker and an additional storage
// for the tracker; the wrapper doesn't do much except of providing methods wrapping
// associated Tracker methods and CliffordCircuit methods into one method, and
// connecting the tracker with the additional storage on measurements
let mut circ = TrackedCircuit {
    // a circuit that does nothing; it could be a real simulator with methods that build
    // up the circuit
    circuit: DummyCircuit {},
    // our tracker
    tracker: Frames::init(3),
    // an additional storage to store the Pauli stacks from measured qubits; here we
    // choose a simple Map, but it could be, for example a storage that puts the data
    // onto files
    storage: Storage::default(),
};

// let's define an additional method to make it easier to use teleported T gates
trait ExtendTrackedCircuit {
    fn teleported_t(&mut self, origin: usize, new: usize);
}
impl ExtendTrackedCircuit for TrackedCircuit<DummyCircuit, Frames, Storage> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn teleported_t(&mut self, origin: usize, new: usize) {
        // this is from the linked paper, naively implement, assuming that we don't know
        // anything about the type of measurement, except that it realizes the T gate
        // with an additional Z correction; in the second part of this example we
        // improve it
        self.tracker.new_qubit(new);
        self.cx(origin, new);
        self.measure_and_store(origin);
        self.track_z(new);
    }
}

// this here is the Toffoli gate decomposed as in the paper from the example description
// above; the input qubits are 0, 1, 2 and the output qubits are 3, 6, 9
circ.teleported_t(0, 3);
circ.teleported_t(1, 4);
circ.h(2);
circ.cx(3, 4);
circ.teleported_t(2, 5);
circ.cx(4, 5);
circ.teleported_t(4, 6);
circ.teleported_t(5, 7);
circ.cx(3, 6);
circ.cx(6, 7);
circ.cx(3, 6);
circ.teleported_t(7, 8);
circ.cx(6, 8);
circ.cx(3, 6);
circ.teleported_t(8, 9);
circ.cx(6, 9);
circ.h(9);
let map = [0, 1, 2, 4, 5, 7, 8];

// let's check out the result;
// these are the three output qubits
assert_eq!(
    vec![
        (3, &PauliStack::try_from_str("0000000", "1101010").unwrap()),
        (6, &PauliStack::try_from_str("0000000", "0001111").unwrap()),
        (9, &PauliStack::try_from_str("0000001", "0000000").unwrap()),
    ],
    circ.tracker.as_storage().sort_by_key()
);
// and these are the other qubits, which have been put into the additional storage, as
// soon as they have been measured; putting them into the additional storage saves
// unnecessary zeros in their Pauli stacks
assert_eq!(
    vec![
        (0, &PauliStack::try_from_str("", "").unwrap()),
        (1, &PauliStack::try_from_str("0", "0").unwrap()),
        (2, &PauliStack::try_from_str("00", "00").unwrap()),
        (4, &PauliStack::try_from_str("000", "011").unwrap()),
        (5, &PauliStack::try_from_str("0000", "0010").unwrap()),
        (7, &PauliStack::try_from_str("00000", "00001").unwrap()),
        (8, &PauliStack::try_from_str("000000", "000001").unwrap())
    ],
    circ.storage.sort_by_key()
);

// let's view the dependency graph: we need to do some prework
// first put everything into the storage
circ.measure_and_store_all();
// to make the assert work we need a storage with an determinitic iterator; you probably
// don't need to do this in a real application
let storage = BufferedVector::from(
    circ.storage.into_sorted_by_key()
    .into_iter()
    .map(|(_, stack)| stack)
    .collect::<Vec<_>>()
);
// now the graph:
assert_eq!(
    dependency_graph::create_dependency_graph(Iterable::iter_pairs(&storage), &map),
    vec![
        vec![(0, vec![]), (1, vec![]), (2, vec![])],
        vec![(5, vec![2]), (4, vec![1, 2])],
        vec![(7, vec![5])],
        vec![(8, vec![7]), (3, vec![0, 4, 7])],
        vec![(6, vec![4, 8]), (9, vec![8])],
    ]
);
# }
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(not(all(feature = "circuit", feature = "bit-vec")))]
# fn main() {}
```
As noted in the code above, our teleported T gate is a little bit naive. When looking
into more details of this [paper], one can see that the measurement that we perform for
the teleported T gate actually anti-commutes with the Z gate. Importantly, this is also
true if we have some X corrections, because the X corrections would only change the
angle in the measurement. This means that we do not have to change the measurement to
compensate the Z corrections, instead we can account for them via post-processing - they
only flip the sign of the measurement outcome - instead of changing the measurement. We
can do this directly in the Pauli tracker: Flipping the sign of the measurement outcome,
depending on the previous measurements, is equivalent to completely removing the Z
corrections from the teleported qubit and instead putting them onto the new qubit as Z
corrections, since the teleportation introduces a Z correction. This can be achieved
with [Tracker::move_z_to_z]:
```rust
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(all(feature = "circuit", feature = "bit-vec"))]
# fn main() {
# use pauli_tracker::{
#     pauli::{self, Pauli},
#     circuit::{CliffordCircuit, DummyCircuit, TrackedCircuit},
#     collection::{Map, BufferedVector, Iterable, Full, Init},
#     tracker::{Tracker, frames::{Frames, dependency_graph}},
# };
# type BoolVec = bit_vec::BitVec;
# type PauliStack = pauli::PauliStack<BoolVec>;
# type Storage = Map<PauliStack>;
# let mut circ = TrackedCircuit {
#     circuit: DummyCircuit {},
#     tracker: Frames::<Storage>::init(3),
#     storage: Storage::default(),
# };
# trait ExtendTrackedCircuit {
#     fn teleported_t(&mut self, origin: usize, new: usize);
# }
// ... same as before ...

impl ExtendTrackedCircuit for TrackedCircuit<DummyCircuit, Frames<Storage>, Storage> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn teleported_t(&mut self, origin: usize, new: usize) {
        self.tracker.new_qubit(new);
        self.cx(origin, new);
        // the "movement" of previous Z corrections; note that this does nothing with
        // the circuit, it effects only the tracker
        self.move_z_to_z(origin, new);
        self.measure_and_store(origin);
        self.track_z(new);
    }
}

# circ.teleported_t(0, 3);
# circ.teleported_t(1, 4);
# circ.h(2);
# circ.cx(3, 4);
# circ.teleported_t(2, 5);
# circ.cx(4, 5);
# circ.teleported_t(4, 6);
# circ.teleported_t(5, 7);
# circ.cx(3, 6);
# circ.cx(6, 7);
# circ.cx(3, 6);
# circ.teleported_t(7, 8);
# circ.cx(6, 8);
# circ.cx(3, 6);
# circ.teleported_t(8, 9);
# circ.cx(6, 9);
# circ.h(9);
# let map = [0, 1, 2, 4, 5, 7, 8];
// ...

// the output qubits
assert_eq!(
    vec![
        (3, &PauliStack::try_from_str("0000000", "1001110").unwrap()),
        (6, &PauliStack::try_from_str("0000000", "0101101").unwrap()),
        (9, &PauliStack::try_from_str("0010111", "0000000").unwrap()),
    ],
    circ.tracker.as_storage().sort_by_key()
);
// the other qubits; moving the Z corrections literally removed them from memory
assert_eq!(
    vec![
        (0, &PauliStack::try_from_str("", "").unwrap()),
        (1, &PauliStack::try_from_str("0", "").unwrap()),
        (2, &PauliStack::try_from_str("00", "").unwrap()),
        (4, &PauliStack::try_from_str("000", "").unwrap()),
        (5, &PauliStack::try_from_str("0000", "").unwrap()),
        (7, &PauliStack::try_from_str("00000", "").unwrap()),
        (8, &PauliStack::try_from_str("000000", "").unwrap())
    ],
    circ.storage.sort_by_key()
);

# circ.measure_and_store_all();
# let storage = BufferedVector::from(
#     circ.storage.into_sorted_by_key()
#       .into_iter()
#       .map(|(_, stack)| stack)
#       .collect::<Vec<_>>()
# );
// ...

assert_eq!(
    dependency_graph::create_dependency_graph(&storage, &map),
    vec![
        vec![
            (0, vec![]), (1, vec![]), (2, vec![]),
            (4, vec![]), (5, vec![]), (7, vec![]), (8, vec![]),
        ],
        vec![(3, vec![0, 4, 5, 7]), (6, vec![1, 4, 5, 8]), (9, vec![2, 5, 7, 8])],
    ]
);
// -> only two layers instead of 5 layers!
# }
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(not(all(feature = "circuit", feature = "bit-vec")))]
# fn main() {}
```

[bitvec]: https://docs.rs/bitvec/latest/bitvec/index.html
[bitvec_simd::BitVec]: https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html
[bitvec::vec::BitVec]: https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html
[bit_vec::BitVec]: https://docs.rs/bit-vec/latest/bit_vec/struct.BitVec.html
[bit-vec]: https://docs.rs/bit-vec/latest/bit_vec/index.html
[BooleanVector]: boolean_vector::BooleanVector
[create_dependency_graph]: tracker::frames::dependency_graph::create_dependency_graph
[Frames]: tracker::frames::Frames
[paper]: https://arxiv.org/abs/2209.07345v2
[rand]: https://crates.io/crates/rand
[README]: https://github.com/taeruh/pauli_tracker
[serde]: https://docs.rs/serde/latest/serde/
[SimdBitVec]: boolean_vector::bitvec_simd::SimdBitVec
[smallvec]: https://docs.rs/smallvec/1.10.0/smallvec/
[Tracker::move_z_to_z]: tracker::Tracker::move_z_to_z
