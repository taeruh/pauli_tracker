/*!
Track Pauli gates when executing a circuit.

This module provides the [Live] tracker. Compare the documentation in
[frames](super::frames). The difference between the [Live] tracker and the [Frames] is
that while the [Frames] tracker stores each Pauli frame, the [Live] tracker adds them
up.

Usually you want to use this tracker during execution of a circuit and track *all*
Paulis via `track_x/y/z` methods.

[Frames]: super::frames::Frames
*/

use std::mem;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    unwrap_get_mut,
    unwrap_get_two_mut,
    MissingBit,
    PauliString,
    Tracker,
};
use crate::{
    collection::{
        Base,
        Init,
    },
    pauli::Pauli,
};

// todo: make it generic and also do it with a hashmap

/// An implementor of [Tracker] that tracks Pauli gates at runtime.
///
/// Compare the [module documentation](super::live). To be useful, the generic `Storage`
/// type should at least implement [Base], with implementors of [Pauli] as elements.
// I'm not sure what the most efficient inner type would be here, Vec<bool>, Vec<Pauli>,
// BitVec, ...
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Live<Storage> {
    storage: Storage,
}

impl<S> From<S> for Live<S> {
    fn from(value: S) -> Self {
        Self { storage: value }
    }
}

impl<S> AsRef<S> for Live<S> {
    fn as_ref(&self) -> &S {
        &self.storage
    }
}

impl<T> Live<T> {
    /// Creates a new [Live] tracker with the given storage.
    pub fn new(storage: T) -> Self {
        Self { storage }
    }

    /// Returns the inner storage.
    pub fn into(self) -> T {
        self.storage
    }
}

impl<T: Init> Init for Live<T> {
    fn init(len: usize) -> Self {
        Self { storage: T::init(len) }
    }
}

impl<S, T> Live<S>
where
    S: Base<TB = T>,
{
    /// Returns a mutable reference to an element at index. Returns [None] if out of
    /// bounds.
    pub fn get_mut(&mut self, bit: usize) -> Option<&mut T> {
        self.storage.get_mut(bit)
    }

    /// Returns a reference to an element at index. Returns [None] if out of bounds.
    pub fn get(&self, bit: usize) -> Option<&T> {
        self.storage.get(bit)
    }
}

macro_rules! single {
    ($($name:ident,)*) => {$(
        fn $name(&mut self, bit: usize) {
            unwrap_get_mut!(self.storage, bit, stringify!($name)).$name()
        }
    )*};
}

macro_rules! movements {
    ($(($name:ident, $plus:ident, $set:ident),)*) => {$(
        fn $name(&mut self, source: usize, destination: usize) {
            let (s, d) =
                unwrap_get_two_mut!(self.storage, source, destination, stringify!($name));
            d.$plus(s);
            s.$set(false);
        }
    )*};
}

/// Note that the inner storage type is basically a vector. Therefore, the it may
/// contain buffer qubits, even though they were not explicitly initialized.
impl<S, P> Tracker for Live<S>
where
    S: Base<TB = P>,
    P: Pauli + Clone,
{
    type Stack = P;
    type Pauli = P;

    movements!(
        (move_x_to_x, xpx, set_x),
        (move_x_to_z, zpx, set_x),
        (move_z_to_x, xpz, set_z),
        (move_z_to_z, zpz, set_z),
    );

    fn new_qubit(&mut self, bit: usize) -> Option<Self::Stack> {
        self.storage.insert(bit, P::I)
    }

    fn track_pauli(&mut self, bit: usize, pauli: Self::Pauli) {
        if let Some(p) = self.storage.get_mut(bit) {
            p.add(pauli)
        }
    }
    fn track_pauli_string(&mut self, string: PauliString<Self::Pauli>) {
        for (bit, pauli) in string {
            if let Some(p) = self.storage.get_mut(bit) {
                p.add(pauli)
            }
        }
    }

    single!(h, s, sx,);
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, bit_a, bit_b, "cz");
        a.zpx(b);
        b.zpx(a);
    }

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.storage, control, target, "cx");
        t.xpx(c);
        c.zpz(t);
    }

    fn cy(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.storage, control, target, "cx");
        // cf. comment in frames implementation
        c.zpz(t);
        c.zpx(t);
        t.zpx(c);
        t.xpx(c);
    }

    fn swap(&mut self, control: usize, target: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, control, target, "swap");
        mem::swap(a, b)
    }

    fn iswap(&mut self, control: usize, target: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, control, target, "swap");
        mem::swap(a, b);
        let copy = a.get_x() ^ b.get_x();
        a.set_z(a.get_z() ^ copy);
        b.set_z(b.get_z() ^ copy);
    }

    fn measure(&mut self, bit: usize) -> Result<Self::Stack, MissingBit> {
        self.get_mut(bit).ok_or(MissingBit(bit)).cloned()
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::{
        collection::BufferedVector,
        pauli::{
            PauliDense,
            PauliEnum,
            PauliTuple,
        },
    };

    trait Pw: Pauli + Copy + Clone + Default + Into<PauliDense> + From<PauliDense> {}
    type Live<P> = super::Live<BufferedVector<P>>;

    mod single_actions {
        use super::*;
        use crate::tracker::tests::utils::{
            self,
            SingleAction,
            SingleResults,
            N_SINGLES,
        };

        type Action<P> = SingleAction<Live<P>>;

        #[cfg_attr(coverage_nightly, coverage(off))]
        fn runner<P: Pw>(action: Action<P>, result: SingleResults) {
            for (input, check) in (0u8..).zip(result.1) {
                let mut tracker = Live::<P>::init(2);
                tracker.track_pauli_string(utils::single_init(input));
                (action)(&mut tracker, 0);
                assert_eq!(
                    P::into(*tracker.storage.get(0).unwrap()).storage(),
                    check,
                    "{}, {}",
                    result.0,
                    input
                );
            }
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
        pub(super) fn run<P: Pw>() {
            let actions: [Action<P>; N_SINGLES] = utils::single_actions!(Live<P>);
            utils::single_check(runner, actions);
        }
    }

    mod double_actions {
        use super::*;
        use crate::tracker::tests::utils::{
            self,
            DoubleAction,
            DoubleResults,
            N_DOUBLES,
        };

        type Action<P> = DoubleAction<Live<P>>;

        #[cfg_attr(coverage_nightly, coverage(off))]
        fn runner<P: Pw>(action: Action<P>, result: DoubleResults) {
            for (input, check) in (0u8..).zip(result.1) {
                let mut tracker = Live::init(2);
                tracker.track_pauli_string(utils::double_init(input));
                (action)(&mut tracker, 0, 1);
                let output = utils::double_output(tracker.storage);
                assert_eq!(output, check, "{}, {}", result.0, input);
            }
        }

        pub(super) fn run<T: Pw>() {
            let actions: [Action<T>; N_DOUBLES] = utils::double_actions!(Live<T>);

            utils::double_check(runner, actions);
        }
    }

    macro_rules! test_actions {
        ($(($pauli:ty, $module:ident),)*) => {$(
            mod $module {
                use super::test;
                #[rustfmt::skip]
                use super::{double_actions, single_actions, Pw, $pauli};
                impl Pw for $pauli {}
                #[test]
                fn single_actions() {
                    single_actions::run::<$pauli>();
                }
                #[test]
                fn double_actions() {
                    double_actions::run::<$pauli>();
                }
            }
        )*};
    }

    test_actions!(
        (PauliDense, pauli_dense),
        (PauliTuple, pauli_tuple),
        (PauliEnum, pauli_enum),
    );

    #[test]
    fn new_qubit_and_measure() {
        let mut tracker = Live::<PauliTuple>::init(1);
        tracker.track_x(0);
        assert_eq!(tracker.new_qubit(0), Some(PauliTuple::X));
        assert_eq!(tracker.new_qubit(1), None);
        tracker.track_y(0);
        assert_eq!(tracker.as_ref().0, vec![PauliTuple::Y, PauliTuple::I]);
        assert_eq!(tracker.measure(0), Ok(PauliTuple::Y));
        assert_eq!(tracker.new_qubit(3), None);
        assert_eq!(
            *tracker.as_ref().0,
            vec![
                PauliTuple::new_y(),
                PauliTuple::new_i(),
                PauliTuple::new_i(),
                PauliTuple::new_i()
            ]
        );
    }

    //
}
