/*!
This module provides [Tracker]s that are similar to the ones in [frames](super::frames),
with the major difference that there's effectively only one frames, which adds up
multiple tracked Paulis.
*/

use std::{
    self,
    cmp::Ordering,
    iter,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    unwrap_get_mut,
    unwrap_get_two_mut,
    MissingStack,
    PauliString,
    Tracker,
};
use crate::{
    pauli::Pauli,
    slice_extension::GetTwoMutSlice,
};

// todo: make it generic and also do it with a hashmap

/// An implementor of [Tracker], similar to [Frames](super::frames::Frames), with the
/// difference, that instead of storing each Pauli frame, it adds the Pauli frames (mod
/// 2).
// I'm not sure what the most efficient inner type would be here, Vec<bool>, Vec<Pauli>,
// BitVec, ...
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiveVector<T> {
    inner: Vec<T>,
}

impl<T> From<Vec<T>> for LiveVector<T> {
    fn from(value: Vec<T>) -> Self {
        Self { inner: value }
    }
}

impl<T> From<LiveVector<T>> for Vec<T> {
    fn from(value: LiveVector<T>) -> Self {
        value.inner
    }
}

impl<T> AsRef<Vec<T>> for LiveVector<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T> LiveVector<T> {
    /// Returns a reference to an element at index. Returns [None] if out of bounds.
    pub fn get(&self, bit: usize) -> Option<&T> {
        self.inner.get(bit)
    }
    /// Returns a mutable reference to an element at index. Returns [None] if out of
    /// bounds.
    pub fn get_mut(&mut self, bit: usize) -> Option<&mut T> {
        self.inner.get_mut(bit)
    }
}

macro_rules! single {
    ($($name:ident,)*) => {$(
        fn $name(&mut self, bit: usize) {
            unwrap_get_mut!(self.inner, bit, stringify!($name)).$name()
        }
    )*};
}

macro_rules! movements {
    ($(($name:ident, $plus:ident, $set:ident),)*) => {$(
        fn $name(&mut self, source: usize, destination: usize) {
            let (s, d) =
                unwrap_get_two_mut!(self.inner, source, destination, stringify!($name));
            d.$plus(s);
            s.$set(false);
        }
    )*};
}

/// Note that the inner storage type is basically a vector. Therefore, the it may
/// contain buffer qubits, even though they were not explicitly initialized.
impl<T> Tracker for LiveVector<T>
where
    T: Pauli + Clone,
{
    type Stack = T;
    type Pauli = T;

    movements!(
        (move_x_to_x, xpx, set_x),
        (move_x_to_z, zpx, set_x),
        (move_z_to_x, xpz, set_z),
        (move_z_to_z, zpz, set_z),
    );

    fn init(num_bits: usize) -> Self {
        LiveVector {
            inner: vec![T::new_i(); num_bits],
        }
    }

    fn new_qubit(&mut self, bit: usize) -> Option<usize> {
        let len = self.inner.len();
        match bit.cmp(&len) {
            Ordering::Less => Some(bit),
            Ordering::Equal => {
                self.inner.push(T::new_i());
                None
            }
            Ordering::Greater => {
                let diff = bit - len + 1;
                self.inner.try_reserve(diff).unwrap_or_else(|e| {
                    panic!("error when trying to reserve enough memory: {e}")
                });
                self.inner.extend(iter::repeat(T::new_i()).take(diff));
                None
            }
        }
    }

    fn track_pauli(&mut self, bit: usize, pauli: T) {
        if let Some(p) = self.inner.get_mut(bit) {
            p.add(pauli)
        }
    }
    fn track_pauli_string(&mut self, string: PauliString<T>) {
        for (bit, pauli) in string {
            if let Some(p) = self.inner.get_mut(bit) {
                p.add(pauli)
            }
        }
    }

    single!(h, s,);

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.inner, control, target, "cx");
        t.xpx(c);
        c.zpz(t);
    }
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.inner, bit_a, bit_b, "cz");
        a.zpx(b);
        b.zpx(a);
    }

    fn measure(&mut self, bit: usize) -> Result<Self::Stack, MissingStack> {
        self.get(bit).ok_or(MissingStack { bit }).cloned()
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::pauli::{
        PauliDense,
        PauliTuple,
    };

    trait P: Pauli + Copy + Into<PauliDense> + From<PauliDense> {}

    mod single_actions {
        use super::*;
        use crate::tracker::test::impl_utils::{
            self,
            SingleAction,
            SingleResults,
            N_SINGLES,
        };

        type Action<T> = SingleAction<LiveVector<T>>;

        #[cfg_attr(coverage_nightly, no_coverage)]
        fn runner<T: P>(action: Action<T>, result: SingleResults) {
            for (input, check) in (0u8..).zip(result.1) {
                let mut tracker = LiveVector::init(2);
                tracker.track_pauli_string(impl_utils::single_init(input));
                (action)(&mut tracker, 0);
                assert_eq!(
                    T::into(*tracker.inner.get(0).unwrap()).storage(),
                    check,
                    "{}, {}",
                    result.0,
                    input
                );
            }
        }

        #[cfg_attr(coverage_nightly, no_coverage)]
        pub(super) fn run<T: P>() {
            let actions: [Action<T>; N_SINGLES] = [LiveVector::h, LiveVector::s];
            impl_utils::single_check(runner, actions);
        }
    }

    mod double_actions {
        use super::*;
        use crate::tracker::test::impl_utils::{
            self,
            DoubleAction,
            DoubleResults,
            N_DOUBLES,
        };

        type Action<T> = DoubleAction<LiveVector<T>>;

        #[cfg_attr(coverage_nightly, no_coverage)]
        fn runner<T: P>(action: Action<T>, result: DoubleResults) {
            for (input, check) in (0u8..).zip(result.1) {
                let mut tracker = LiveVector::init(2);
                tracker.track_pauli_string(impl_utils::double_init(input));
                (action)(&mut tracker, 0, 1);
                let output =
                    impl_utils::double_output(tracker.inner.into_iter().enumerate());
                assert_eq!(output, check, "{}, {}", result.0, input);
            }
        }

        pub(super) fn run<T: P>() {
            let actions: [Action<T>; N_DOUBLES] = [
                LiveVector::cx,
                LiveVector::cz,
                LiveVector::move_x_to_x,
                LiveVector::move_x_to_z,
                LiveVector::move_z_to_x,
                LiveVector::move_z_to_z,
            ];

            impl_utils::double_check(runner, actions);
        }
    }

    macro_rules! test_actions {
        ($(($pauli:ty, $module:ident),)*) => {$(
            mod $module {
                use super::test;
                #[rustfmt::skip]
                use super::{double_actions, single_actions, P, $pauli};
                impl P for $pauli {}
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

    test_actions!((PauliDense, pauli_dense), (PauliTuple, pauli_tuple),);

    #[test]
    fn new_qubit_and_measure() {
        let mut tracker = LiveVector::init(1);
        tracker.track_x(0);
        assert_eq!(tracker.new_qubit(0), Some(0));
        assert_eq!(tracker.new_qubit(1), None);
        assert_eq!(*tracker.as_ref(), vec![PauliTuple::new_x(), PauliTuple::new_i()]);
        assert_eq!(tracker.measure(0), Ok(PauliTuple::new_x()));
        assert_eq!(tracker.new_qubit(3), None);
        assert_eq!(
            *tracker.as_ref(),
            vec![
                PauliTuple::new_x(),
                PauliTuple::new_i(),
                PauliTuple::new_i(),
                PauliTuple::new_i()
            ]
        );
    }

    //
}
