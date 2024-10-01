// trying to do something which is done in Paler et al's paper

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use pauli_tracker::{
    collection::{self, Init},
    pauli::{self, Pauli},
    tracker::{Tracker, live},
};
use rand::{
    RngCore, SeedableRng, distributions::Uniform, prelude::Distribution, seq::index,
};
use rand_pcg::Pcg64;

const NUM_BITS: usize = 5100;
const NUM_OPS: usize = 50000;

type PauliCode = pauli::PauliDense;
// type PauliReturn = PauliCode;
type PauliReturn = pauli::PauliEnum;
// type Live<T> = live::Live<collection::NaiveVector<T>>;
// type Live<T> = live::Live<collection::BufferedVector<T>>;
type Live<T> = live::Live<collection::Map<T>>;
// type Live<T> =
//     live::Live<collection::Map<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>;

struct Circuit {
    num_bits: usize,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    TrackX(usize),
    TrackY(usize),
    TrackZ(usize),
    H(usize),
    S(usize),
    Cz(usize, usize),
    I(usize),
    X(usize),
    Y(usize),
    Z(usize),
    Sdg(usize),
    Hxy(usize),
    Hyz(usize),
    Sx(usize),
    Sy(usize),
    Sz(usize),
    Sxdg(usize),
    Sydg(usize),
    Szdg(usize),
    Cx(usize, usize),
    Cy(usize, usize),
    Swap(usize, usize),
    ISwap(usize, usize),
    ISwapdg(usize, usize),
    MoveXX(usize, usize),
    MoveXZ(usize, usize),
    MoveZX(usize, usize),
    MoveZZ(usize, usize),
}

impl Circuit {
    fn new(num_bits: usize, num_ops: usize, rng: &mut impl RngCore) -> Self {
        use Instruction::*;
        let mut instructions = Vec::with_capacity(num_ops);
        let gate_dist = Uniform::new(0, 28);
        let bit_dist = Uniform::new(0, num_bits);

        fn double_idx(
            num_bits: usize,
            dist: &Uniform<usize>,
            rng: &mut impl RngCore,
        ) -> (usize, usize) {
            // maybe one should directly use index sample; who knows, doesn't matter
            let a = dist.sample(rng);
            let b = dist.sample(rng);
            if a == b {
                let ab = index::sample(rng, num_bits, 2);
                (ab.index(0), ab.index(1))
            } else {
                (a, b)
            }
        }

        for _ in 0..num_ops {
            match gate_dist.sample(rng) {
                0 => instructions.push(TrackX(bit_dist.sample(rng))),
                1 => instructions.push(TrackY(bit_dist.sample(rng))),
                2 => instructions.push(TrackZ(bit_dist.sample(rng))),
                3 => instructions.push(H(bit_dist.sample(rng))),
                4 => instructions.push(S(bit_dist.sample(rng))),
                5 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(Cz(a, b))
                },
                6 => instructions.push(I(bit_dist.sample(rng))),
                7 => instructions.push(X(bit_dist.sample(rng))),
                8 => instructions.push(Y(bit_dist.sample(rng))),
                9 => instructions.push(Z(bit_dist.sample(rng))),
                10 => instructions.push(Sdg(bit_dist.sample(rng))),
                11 => instructions.push(Hxy(bit_dist.sample(rng))),
                12 => instructions.push(Hyz(bit_dist.sample(rng))),
                13 => instructions.push(Sx(bit_dist.sample(rng))),
                14 => instructions.push(Sy(bit_dist.sample(rng))),
                15 => instructions.push(Sz(bit_dist.sample(rng))),
                16 => instructions.push(Sxdg(bit_dist.sample(rng))),
                17 => instructions.push(Sydg(bit_dist.sample(rng))),
                18 => instructions.push(Szdg(bit_dist.sample(rng))),
                19 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(Cx(a, b))
                },
                20 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(Cy(a, b))
                },
                21 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(Swap(a, b))
                },
                22 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(ISwap(a, b))
                },
                23 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(ISwapdg(a, b))
                },
                24 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(MoveXX(a, b))
                },
                25 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(MoveXZ(a, b))
                },
                26 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(MoveZX(a, b))
                },
                27 => {
                    let (a, b) = double_idx(num_bits, &bit_dist, rng);
                    instructions.push(MoveZZ(a, b))
                },
                _ => {
                    unreachable!()
                },
            }
        }
        Self { num_bits, instructions }
    }

    fn run<P: Pauli + Clone + Default + Into<PauliReturn>>(
        &mut self,
    ) -> Vec<PauliReturn> {
        use Instruction::*;
        let mut tracker = Live::<P>::init(self.num_bits);
        for instruction in self.instructions.iter() {
            match *instruction {
                TrackX(i) => tracker.track_x(i),
                TrackY(i) => tracker.track_y(i),
                TrackZ(i) => tracker.track_z(i),
                H(i) => tracker.h(i),
                S(i) => tracker.s(i),
                Cz(i, j) => tracker.cz(i, j),
                I(i) => tracker.id(i),
                X(i) => tracker.x(i),
                Y(i) => tracker.y(i),
                Z(i) => tracker.z(i),
                Sdg(i) => tracker.sdg(i),
                Hxy(i) => tracker.hxy(i),
                Hyz(i) => tracker.hyz(i),
                Sx(i) => tracker.sx(i),
                Sy(i) => tracker.sy(i),
                Sz(i) => tracker.sz(i),
                Sxdg(i) => tracker.sxdg(i),
                Sydg(i) => tracker.sydg(i),
                Szdg(i) => tracker.szdg(i),
                Cx(i, j) => tracker.cx(i, j),
                Cy(i, j) => tracker.cy(i, j),
                Swap(i, j) => tracker.swap(i, j),
                ISwap(i, j) => tracker.iswap(i, j),
                ISwapdg(i, j) => tracker.iswapdg(i, j),
                MoveXX(i, j) => tracker.move_x_to_x(i, j),
                MoveXZ(i, j) => tracker.move_x_to_z(i, j),
                MoveZX(i, j) => tracker.move_z_to_x(i, j),
                MoveZZ(i, j) => tracker.move_z_to_z(i, j),
            }
        }
        let mut ret = vec![PauliReturn::default(); self.num_bits];
        for (idx, pauli) in tracker.into_storage().into_iter() {
            ret[idx] = pauli.into();
        }
        ret
    }
}

fn runner(circ: &mut Circuit) {
    // println!("{:?}\n", circ.run::<PauliCode>());
    circ.run::<PauliCode>();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = Pcg64::from_entropy();
    let mut circ = Circuit::new(NUM_BITS, NUM_OPS, &mut rng);
    c.bench_function("paler", |b| b.iter(|| runner(black_box(&mut circ))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
