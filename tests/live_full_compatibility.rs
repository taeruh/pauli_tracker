#![cfg(feature = "circuit")]

use std::fmt::Debug;

use bit_vec::BitVec;
use pauli_tracker::{
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
                PauliVec,
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

const INIT: usize = 2;
proptest! {
    #![proptest_config(Config {
        cases: 4,
        // proptest! just overwrites (see source code); it doesn't really matter, except
        // that we get a warning but that is ok; we could solve it by manually doing
        // what proptest! does (the basics are straightforward, but it also does some
        // nice things that are not straightforward)
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource(
            "regressions",
        ))),
        ..Default::default()
    })]
    #[test]
    #[ignore = "run it explicitly"]
    fn live_full_compatibility(ops in vec_operation(70)) {
        // println!("{:?}", config);
        let mut generator = Generator::new(INIT, ops);
        let mut circuit = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<Map>::init(INIT),
            storage: Map::default(),
        };
        let mut measurements = Ignore {};
        generator.apply(&mut circuit, &mut measurements);
        circuit.tracker.measure_and_store_all(&mut circuit.storage);

        // println!("{:?}", generator.operations);
        // println!(
        //     "{:?}",
        //     pauli_tracker::tracker::frames::storage::sort_by_bit(&circuit.storage)
        // );

        generator.reinit(INIT);
        let mut live_circuit = TrackedCircuit {
            circuit: RandomMeasurementCircuit {},
            tracker: LiveVector::init(INIT),
            storage: (),
        };
        let mut measurements = Vec::<bool>::new();
        generator.apply(&mut live_circuit, &mut measurements);
        // println!("{:?}", measurements);
        // println!("{:?}", live_circuit.tracker);

        let mut check = vec![Pauli::new_i(); generator.used];
        for (i, pauli) in circuit.storage.iter() {
            check[*i].set_storage(sum_up(pauli, &measurements));
        }
        let check: LiveVector = check.into();
        // println!("{:?}", a);

        assert_eq!(check, live_circuit.tracker);
    }
}

fn sum_up(pauli: &PauliVec, measurements: &[bool]) -> u8 {
    fn inner(bit_vec: &BitVec, measurements: &[bool]) -> u8 {
        bit_vec
            .iter()
            .enumerate()
            .filter_map(|(i, f)| if measurements[i] { Some(f as u8) } else { None })
            .sum::<u8>()
            % 2
    }
    inner(&pauli.right, measurements) + inner(&pauli.left, measurements) * 2
}

trait Measurements<T: ExtendCircuit> {
    fn store(&mut self, result: T::Output);
}
struct Ignore {}
impl Measurements<TrackedCircuit<DummyCircuit, Frames<Map>, Map>> for Ignore {
    fn store(&mut self, _: ()) {}
}
impl Measurements<TrackedCircuit<RandomMeasurementCircuit, LiveVector, ()>>
    for Vec<bool>
{
    fn store(&mut self, result: bool) {
        self.push(result);
    }
}

trait ExtendCircuit {
    type Output;
    fn z_rotation_teleportation(&mut self, origin: usize, new: usize) -> Self::Output;
    fn new_qubit(&mut self, bit: usize);
}
impl ExtendCircuit for TrackedCircuit<DummyCircuit, Frames<Map>, Map> {
    type Output = ();
    fn z_rotation_teleportation(&mut self, origin: usize, new: usize) {
        self.tracker.new_qubit(new);
        self.cx(origin, new);
        self.move_z_to_z(origin, new);
        self.measure_and_store(origin).unwrap();
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

struct Generator {
    used: usize,
    memory: Vec<usize>,
    operations: Vec<Operation>,
}

impl Generator {
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
        fn idx(bit: usize, gen: &Generator) -> usize {
            gen.memory[bit % gen.memory.len()]
        }
        fn double_idx(
            bit_a: usize,
            bit_b: usize,
            gen: &Generator,
        ) -> Option<(usize, usize)> {
            let len = gen.memory.len();
            if len == 1 {
                return None;
            }
            let a = bit_a % len;
            let mut b = bit_b % len;
            if a == b {
                b = (bit_b + 1) % len;
            }
            Some((gen.memory[a], gen.memory[b]))
        }

        for op in self.operations.iter() {
            if self.memory.is_empty() {
                Self::new_qubit(&mut self.memory, &mut self.used, circuit)
            }

            match *op {
                Operation::X(b) => circuit.x(idx(b, self)),
                Operation::Y(b) => circuit.y(idx(b, self)),
                Operation::Z(b) => circuit.z(idx(b, self)),
                Operation::H(b) => circuit.h(idx(b, self)),
                Operation::S(b) => circuit.s(idx(b, self)),
                Operation::CX(a, b) => {
                    match double_idx(a, b, self) {
                        Some((a, b)) => circuit.cx(a, b),
                        None => continue,
                    };
                }
                Operation::CZ(a, b) => match double_idx(a, b, self) {
                    Some((a, b)) => circuit.cz(a, b),
                    None => continue,
                },
                Operation::RZ(b) => {
                    measurements.store(
                        circuit.z_rotation_teleportation(idx(b, self), self.used),
                    );
                    let i = b % self.memory.len();
                    self.memory[i] = self.used;
                    self.used += 1;
                }
                Operation::Measure(b) => {
                    circuit.measure(idx(b, self));
                    self.memory.swap_remove(b % self.memory.len());
                }
                Operation::NewQubit(_) => {
                    Self::new_qubit(&mut self.memory, &mut self.used, circuit)
                }
            }
        }
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

fn operation() -> impl Strategy<Value = Operation> {
    prop_oneof![
        1 => any::<usize>().prop_map(Operation::X),
        1 => any::<usize>().prop_map(Operation::Y),
        1 => any::<usize>().prop_map(Operation::Z),
        1 => any::<usize>().prop_map(Operation::H),
        1 => any::<usize>().prop_map(Operation::S),
        3 => (any::<usize>(), any::<usize>()).prop_map(|(a, b)| Operation::CX(a, b)),
        3 => (any::<usize>(), any::<usize>()).prop_map(|(a, b)| Operation::CZ(a, b)),
        10 => any::<usize>().prop_map(Operation::RZ),
        1 => any::<usize>().prop_map(Operation::Measure),
        1 => any::<usize>().prop_map(Operation::NewQubit),
    ]
}

fn vec_operation(mut max: usize) -> impl Strategy<Value = Vec<Operation>> {
    let mut res = Vec::new();
    while max > 0 {
        res.push(operation());
        max -= 1;
    }
    res
}
