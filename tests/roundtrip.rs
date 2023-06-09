#![cfg(feature = "circuit")]

use std::{
    collections::HashMap,
    fmt::Debug,
};

use pauli_tracker::{
    analyse::{
        self,
        DependencyGraph,
    },
    boolean_vector::BooleanVector,
    circuit::{
        CliffordCircuit,
        DummyCircuit,
        RandomMeasurementCircuit,
        TrackedCircuit,
    },
    pauli::Pauli,
    tracker::{
        frames::{
            storage::{
                Map,
                StackStorage,
            },
            Frames,
        },
        live::LiveVector,
        Tracker,
    },
};
use proptest::{
    arbitrary::any,
    prop_oneof,
    proptest,
    strategy::Strategy,
    test_runner::{
        Config,
        FileFailurePersistence,
    },
};

// type BoolVec = bitvec::vec::BitVec;
// type BoolVec = pauli_tracker::boolean_vector::bitvec_simd::SimdBitVec;
type BoolVec = bit_vec::BitVec;

type Storage = Map<BoolVec>;
// type PauliVec = pauli::PauliVec<BoolVec>;

const MAX_INIT: usize = 100;
const MAX_OPS: usize = 1000;
// const MAX_INIT: usize = 2;
// const MAX_OPS: usize = 10;
proptest! {
    #![proptest_config(Config {
        // cases: 1,
        // proptest! just overwrites this (see source code); it doesn't really matter,
        // except that we get a warning but that is ok; we could solve it by manually
        // doing what proptest! does (the basics are straightforward, but it also does
        // some nice things that are not straightforward)
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource(
            "regressions",
        ))),
        ..Default::default()
    })]
    #[test]
    #[ignore = "run proptests explicitly"]
    fn prop_roundtrip(init in (0..MAX_INIT), ops in vec_operation(MAX_OPS)) {
        roundtrip(init, ops);
    }
}

// Given some operations, we perform the pauli tracking with Frames and create the
// dependency graph. This graph is checked whether it doesn't promise something wrong
// and whether it is optimal. Then we also track Paulis via LiveVector and check
// whether the results are compatible with results from Frames
fn roundtrip(init: usize, ops: Vec<Operation>) {
    // println!("len:  {}", ops.len());
    // println!("init: {}", init);
    let mut generator = Instructor::new(init, ops);
    let mut circuit = TrackedCircuit {
        circuit: DummyCircuit {},
        tracker: Frames::<Storage>::init(init),
        storage: Map::default(),
    };
    let mut measurements = WhereMeasured(Vec::new());
    generator.apply(&mut circuit, &mut measurements);
    circuit.tracker.measure_and_store_all(&mut circuit.storage);

    let graph = analyse::create_dependency_graph(
        <Storage as StackStorage>::iter(&circuit.storage),
        &measurements.0,
    );
    check_graph(&graph, &circuit.storage, &measurements.0).unwrap();

    // println!("graph: {:?}", graph);
    // println!("graph.len: {}", graph.len());
    // println!("{:?}", measurements.0);

    // println!("{:?}", generator.operations);
    // println!("{:?}\n", storage::sort_by_bit(&circuit.storage));

    generator.reinit(init);
    let mut live_circuit = TrackedCircuit {
        circuit: RandomMeasurementCircuit {},
        tracker: LiveVector::init(init),
        storage: (),
    };
    let mut measurements = ResultMeasured(Vec::new());
    generator.apply(&mut live_circuit, &mut measurements);
    // println!("{:?}", measurements);
    // println!("{:?}", live_circuit.tracker);

    let mut check = vec![Pauli::new_i(); generator.used];
    for (i, pauli) in circuit.storage.iter() {
        check[*i].set_storage(pauli.sum_up(&measurements.0).storage());
    }
    let check: LiveVector = check.into();
    // println!("{:?}", a);

    assert_eq!(check, live_circuit.tracker);
}

// {{ helpers to perform the checks
fn check_graph(
    graph: &DependencyGraph,
    storage: &Storage,
    measurements: &[usize],
) -> Result<(), String> {
    fn check(
        dep: (usize, bool),
        measured: &HashMap<usize, ()>,
        measurements: &[usize],
    ) -> Result<(), String> {
        if !dep.1
            || measured
                .contains_key(measurements.get(dep.0).expect("missing measurement"))
        {
            Ok(())
        } else {
            Err(format!("{dep:?}"))
        }
    }

    fn node_check(
        node: &usize,
        deps: &Vec<usize>,
        storage: &Storage,
        measurements: &[usize],
        measured: &HashMap<usize, ()>,
    ) -> Result<(), String> {
        for dep in deps {
            if !measured.contains_key(dep) {
                return Err("{dep:?}".to_string());
            }
        }
        let pauli = storage.get(node).expect("node does not exist");
        // we explicitly do not xor(left, right), because that's what we are doing
        // in the create_dependency_graph function; here we keep it as simple is
        // possible
        // println!("{:?}", pauli.left);
        for dep in pauli.left.iter_vals().enumerate() {
            check(dep, measured, measurements).map_err(|e| format!("left: {e}"))?
        }
        for dep in pauli.right.iter_vals().enumerate() {
            check(dep, measured, measurements).map_err(|e| format!("right: {e}"))?
        }
        Ok(())
    }

    let mut measured = HashMap::<usize, ()>::new();
    let mut iter = graph.iter().peekable();

    while let Some(this_layer) = iter.next() {
        if let Some(next_layer) = iter.peek() {
            for (node, deps) in *next_layer {
                // if a node in the next_layer could be measured, we fail because then
                // it should be in this_layer, since we want to be optimal
                if node_check(node, deps, storage, measurements, &measured).is_ok() {
                    return Err(format!(
                        "not optimal: {node:?}, {deps:?}, \
                         {measured:?}\n{graph:#?}\n{storage:#?}"
                    ));
                }
            }
        }
        for (node, deps) in this_layer {
            // if a node in this_layer can't be measured, we fail because then the
            // dependency graph would be wrong
            match node_check(node, deps, storage, measurements, &measured) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "not sufficient: {e}\n{node:?}, {deps:?}, \
                         {measured:?}\n{graph:?}\n{storage:#?}"
                    ));
                }
            }
            measured.insert(*node, ());
        }
    }
    Ok(())
}
// }}

// a instructor that defines the a tracking circuit based on some operations generated
// with proptest
struct Instructor {
    used: usize,
    memory: Vec<usize>,
    operations: Vec<Operation>,
}

impl Instructor {
    fn new(init: usize, operations: Vec<Operation>) -> Self {
        Self {
            used: init,
            memory: (0..init).collect(),
            operations,
        }
    }

    fn reinit(&mut self, init: usize) {
        self.used = init;
        self.memory = (0..init).collect();
    }

    fn apply<C, T, S>(
        &mut self,
        circuit: &mut TrackedCircuit<C, T, S>,
        measurements: &mut impl Measurements<TrackedCircuit<C, T, S>>,
    ) where
        C: CliffordCircuit,
        T: Tracker,
        TrackedCircuit<C, T, S>: ExtendCircuit,
    {
        for op in self.operations.iter() {
            // for small circuits, we loose some ops
            if self.memory.is_empty() {
                Self::new_qubit(&mut self.memory, &mut self.used, circuit)
            }
            // println!("{:?}", op);

            match *op {
                Operation::X(b) => circuit.x(self.mem_idx(b)),
                Operation::Y(b) => circuit.y(self.mem_idx(b)),
                Operation::Z(b) => circuit.z(self.mem_idx(b)),
                Operation::H(b) => circuit.h(self.mem_idx(b)),
                Operation::S(b) => circuit.s(self.mem_idx(b)),
                Operation::CX(a, b) => {
                    match self.double_mem_idx(a, b) {
                        Some((a, b)) => circuit.cx(a, b),
                        None => continue,
                    };
                }
                Operation::CZ(a, b) => match self.double_mem_idx(a, b) {
                    Some((a, b)) => circuit.cz(a, b),
                    None => continue,
                },
                Operation::RZ(b) => {
                    measurements.store(
                        self.mem_idx(b),
                        circuit.z_rotation_teleportation(self.mem_idx(b), self.used),
                    );
                    let i = self.idx(b);
                    self.memory[i] = self.used;
                    self.used += 1;
                }
                Operation::Measure(b) => {
                    circuit.measure(self.mem_idx(b));
                    self.memory.swap_remove(b % self.memory.len());
                }
                Operation::NewQubit(_) => {
                    Self::new_qubit(&mut self.memory, &mut self.used, circuit)
                }
            }
        }
    }
    #[inline(always)]
    fn idx(&self, bit: usize) -> usize {
        bit % self.memory.len()
    }
    #[inline]
    fn mem_idx(&self, bit: usize) -> usize {
        self.memory[self.idx(bit)]
    }
    fn double_mem_idx(&self, bit_a: usize, bit_b: usize) -> Option<(usize, usize)> {
        let len = self.memory.len();
        // for small circuits, we loose some ops
        if len == 1 {
            return None;
        }
        let a = bit_a % len;
        let mut b = bit_b % len;
        if a == b {
            // this destroys some randomness; should barely happen for large circuits
            b = (bit_b + 1) % len;
        }
        Some((self.memory[a], self.memory[b]))
    }
    fn new_qubit<C, T, S>(
        memory: &mut Vec<usize>,
        used: &mut usize,
        circuit: &mut TrackedCircuit<C, T, S>,
    ) where
        C: CliffordCircuit,
        T: Tracker,
        TrackedCircuit<C, T, S>: ExtendCircuit,
    {
        circuit.new_qubit(*used);
        memory.push(*used);
        *used += 1;
    }
}

trait Measurements<T: ExtendCircuit> {
    fn store(&mut self, bit: usize, result: T::Output);
}
struct WhereMeasured(Vec<usize>);
impl Measurements<TrackedCircuit<DummyCircuit, Frames<Storage>, Storage>>
    for WhereMeasured
{
    fn store(&mut self, bit: usize, _: ()) {
        self.0.push(bit)
    }
}
struct ResultMeasured(Vec<bool>);
impl Measurements<TrackedCircuit<RandomMeasurementCircuit, LiveVector, ()>>
    for ResultMeasured
{
    fn store(&mut self, _: usize, result: bool) {
        self.0.push(result);
    }
}

trait ExtendCircuit {
    type Output;
    fn z_rotation_teleportation(&mut self, origin: usize, new: usize) -> Self::Output;
    fn new_qubit(&mut self, bit: usize);
}
impl ExtendCircuit for TrackedCircuit<DummyCircuit, Frames<Storage>, Storage> {
    type Output = ();
    fn z_rotation_teleportation(&mut self, origin: usize, new: usize) {
        self.tracker.new_qubit(new);
        self.cx(origin, new);
        self.move_z_to_z(origin, new);
        self.measure_and_store(origin).1.unwrap();
        self.track_z(new);
    }
    fn new_qubit(&mut self, bit: usize) {
        self.tracker.new_qubit(bit);
    }
}
impl ExtendCircuit for TrackedCircuit<RandomMeasurementCircuit, LiveVector, ()> {
    type Output = bool;
    fn z_rotation_teleportation(&mut self, origin: usize, new: usize) -> bool {
        self.tracker.new_qubit(new);
        self.cx(origin, new);
        self.move_z_to_z(origin, new);
        let result = self.circuit.measure(origin);
        if result {
            self.track_z(new);
        };
        result
    }
    fn new_qubit(&mut self, bit: usize) {
        self.tracker.new_qubit(bit);
    }
}

#[derive(Debug)]
enum Operation {
    X(usize),
    Y(usize),
    Z(usize),
    H(usize),
    S(usize),
    CX(usize, usize),
    CZ(usize, usize),
    RZ(usize),
    Measure(usize),
    NewQubit(usize),
}

// the creation of the operations with proptest

fn operation() -> impl Strategy<Value = Operation> {
    prop_oneof![
        1 => any::<usize>().prop_map(Operation::X),
        1 => any::<usize>().prop_map(Operation::Y),
        1 => any::<usize>().prop_map(Operation::Z),
        30 => any::<usize>().prop_map(Operation::H),
        30 => any::<usize>().prop_map(Operation::S),
        30 => (any::<usize>(), any::<usize>()).prop_map(|(a, b)| Operation::CX(a, b)),
        30 => (any::<usize>(), any::<usize>()).prop_map(|(a, b)| Operation::CZ(a, b)),
        100 => any::<usize>().prop_map(Operation::RZ),
        1 => any::<usize>().prop_map(Operation::Measure),
        1 => any::<usize>().prop_map(Operation::NewQubit),
    ]
}

fn fixed_vec_operation(mut max: usize) -> impl Strategy<Value = Vec<Operation>> {
    let mut res = Vec::new();
    while max > 0 {
        res.push(operation());
        max -= 1;
    }
    res
}

fn vec_operation(max: usize) -> impl Strategy<Value = Vec<Operation>> {
    (0..max).prop_flat_map(fixed_vec_operation)
}
