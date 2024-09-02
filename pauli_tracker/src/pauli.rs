/*!
Different representations of Pauli operators.

Throughout this module, we define Pauli operators as products of X and Z operators,
neglecting any phases. Note that it is Y = XZ, up to a phase (and (anti)cyclical). We
basically represent Paulis in their tableau representation, without phases.

We provide three different representations of single Pauli operators:
- [PauliTuple]: Just a tuple of two booleans
- [PauliDense]: The Pauli encoded into a single byte
- [PauliEnum]: The Pauli described as an enum. This very similar to [PauliDense];
  internally, [PauliDense] uses binary operations like '&', '^', etc. and [PauliEnum] uses
  a bunch of match statements.

It probably depends very much on the situation which representation is best. We haven't
performed any good benchmarks (a simple (naive) benchmark shows that it is maybe best
to use [PauliDense] or [PauliTuple] during the tracking and convert it afterwards, if
needed, into [PauliEnum]). If needed one can easily create a custom type that implements
[Pauli].

[PauliStack] is a stack for multiple Pauli operators, which is used in the
[Frames](crate::tracker::frames::Frames) tracker.
*/

macro_rules! const_pauli {
    ($($name:ident,)*) => {$(
        /// Pauli
        #[doc = stringify!($name)]
        /// .
        const $name: Self;
    )*};
}

macro_rules! new_pauli {
    ($(($name:ident, $gate:ident),)*) => {$(
        /// Create a new
        #[doc = stringify!($gate)]
        /// Pauli.
        fn $name() -> Self where Self: Sized {
            Self::$gate
        }
    )*};
}

macro_rules! plus {
    ($(($name:ident, $left:ident, $right:ident),)*) => {$(
        /// Add `other`'s
        #[doc = stringify!($right)]
        /// component onto `self`'s
        #[doc = stringify!($left)]
        /// component in place.
        fn $name(&mut self, other: &Self);
    )*};
}

/// The interface we need for the Pauli tracking.
///
/// Note that we only implement some of the gate conjugations, since many are redundant;
/// also you may want to implement some of the default gate conjugations directly for
/// performance reasons; compare the documentation of [Tracker].
///
/// [Tracker]: crate::tracker::Tracker
pub trait Pauli {
    const_pauli!(I, X, Y, Z,);
    new_pauli!((new_i, I), (new_x, X), (new_y, Y), (new_z, Z),);

    /// Create a the new Pauli (X if x) * (Z if z), neglecting phases.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// assert_eq!(PauliDense::new_product(false, false), PauliDense::new_i());
    /// assert_eq!(PauliDense::new_product(false, true), PauliDense::new_x());
    /// assert_eq!(PauliDense::new_product(true, false), PauliDense::new_z());
    /// assert_eq!(PauliDense::new_product(true, true), PauliDense::new_y());
    /// # }
    /// ```
    fn new_product(z: bool, x: bool) -> Self;

    /// Multiply `self` with `other` in place (i.e., adding on the tableau
    /// representation).
    fn multiply(&mut self, other: Self);

    /// Add the `other` Pauli to `self` in place.
    #[deprecated(since = "0.4.2", note = "use `multiply` instead")]
    // cannot add default implementation, because that would require a Self: Sized bound
    // which might be a breaking change (maybe?)
    fn add(&mut self, other: Self);

    /// Conjugate the Pauli with the I (identity gate). This does nothing!
    #[inline(always)]
    fn id(&mut self) {}

    /// Conjugate the Pauli with the S gate ignoring phases.
    fn s(&mut self);
    /// Conjugate the Pauli with the H gate ignoring phases.
    fn h(&mut self);
    /// Conjugate the Pauli with the SH gate ignoring phases.
    fn sh(&mut self) {
        self.h();
        self.s();
    }
    /// Conjugate the Pauli with the HS gate ignoring phases.
    fn hs(&mut self) {
        self.s();
        self.h();
    }
    /// Conjugate the Pauli with the SHS gate ignoring phases.
    fn shs(&mut self) {
        self.s();
        self.h();
        self.s();
    }

    plus!((xpx, X, X), (xpz, X, Z), (zpx, Z, X), (zpz, Z, Z),);

    /// Get the Pauli's X component.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let pauli = PauliDense::new_y();
    /// assert_eq!(pauli.get_x(), true);
    /// # }
    /// ```
    fn get_x(&self) -> bool;

    /// Get the Pauli's Z component.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let pauli = PauliDense::new_y();
    /// assert_eq!(pauli.get_z(), true);
    /// # }
    /// ```
    fn get_z(&self) -> bool;

    /// Set whether the Pauli products contains X.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let mut pauli = PauliDense::new_y();
    /// pauli.set_x(false);
    /// assert_eq!(pauli, Pauli::new_z());
    /// # }
    /// ```
    fn set_x(&mut self, x: bool);

    /// Set whether the Pauli products contains Z.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let mut pauli = PauliDense::new_y();
    /// pauli.set_z(false);
    /// assert_eq!(pauli, Pauli::new_x());
    /// # }
    /// ```
    fn set_z(&mut self, z: bool);

    /// Translate into the tableau encoding
    fn tableau_encoding(&self) -> u8;
}

mod dense;
pub use dense::PauliDense;
mod enumlike;
pub use enumlike::PauliEnum;
mod tuple;
pub use tuple::PauliTuple;

impl From<PauliEnum> for PauliDense {
    fn from(pauli: PauliEnum) -> Self {
        // safety: discriminant follows the tableau encoding, so it is < 4
        unsafe { Self::from_unchecked(pauli.discriminant()) }
    }
}
impl From<PauliTuple> for PauliDense {
    fn from(pauli: PauliTuple) -> Self {
        Self::new_product(pauli.get_z(), pauli.get_x())
    }
}

impl From<PauliDense> for PauliEnum {
    fn from(pauli: PauliDense) -> Self {
        pauli.storage().try_into().unwrap_or_else(|e| panic!("{e}"))
    }
}
impl From<PauliTuple> for PauliEnum {
    fn from(pauli: PauliTuple) -> Self {
        Self::new_product(pauli.get_x(), pauli.get_z())
    }
}

impl From<PauliDense> for PauliTuple {
    fn from(pauli: PauliDense) -> Self {
        Self::new_product(pauli.get_z(), pauli.get_x())
    }
}
impl From<PauliEnum> for PauliTuple {
    fn from(pauli: PauliEnum) -> Self {
        Self::new_product(pauli.get_z(), pauli.get_x())
    }
}

pub mod stack;
#[doc(inline)]
pub use stack::PauliStack;

/// Pauli encoding into two bits (ignoring phases).
pub mod tableau_encoding {
    /// Code for the identity.
    pub const I: u8 = 0;
    /// Code for the Pauli X gate.
    pub const X: u8 = 2;
    /// Code for the Pauli Y gate.
    pub const Y: u8 = 3;
    /// Code for the Pauli Z gate.
    pub const Z: u8 = 1;
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;
    trait PauliAssert: Pauli + fmt::Debug + PartialEq + Copy {}
    impl PauliAssert for PauliDense {}
    impl PauliAssert for PauliEnum {}
    impl PauliAssert for PauliTuple {}

    macro_rules! check {
        () => {
            check::<PauliDense>();
            check::<PauliEnum>();
            check::<PauliTuple>();
        };
        (combinations) => {
            check::<PauliDense, PauliEnum>();
            check::<PauliDense, PauliTuple>();
            check::<PauliEnum, PauliDense>();
        };
    }

    #[test]
    fn consistency() {
        fn check<T: PauliAssert>() {
            let mapping = [
                (T::I, &T::new_i as &dyn Fn() -> T, (false, false), tableau_encoding::I),
                (T::Z, &T::new_z as &dyn Fn() -> T, (true, false), tableau_encoding::Z),
                (T::X, &T::new_x as &dyn Fn() -> T, (false, true), tableau_encoding::X),
                (T::Y, &T::new_y as &dyn Fn() -> T, (true, true), tableau_encoding::Y),
            ];
            for (t_const, fun, (prod_z, prod_x), tableau_const) in mapping {
                assert_eq!(t_const, fun());
                assert_eq!(t_const, T::new_product(prod_z, prod_x));
                assert_eq!(t_const.tableau_encoding(), tableau_const);
            }
        }
        check!();
    }

    #[test]
    fn conversions() {
        fn check<A, B>()
        where
            A: PauliAssert + From<B>,
            B: PauliAssert + From<A>,
        {
            for (a, b) in
                [A::I, A::Z, A::X, A::Y].into_iter().zip([B::I, B::Z, B::X, B::Y])
            {
                assert_eq!(A::from(b), a);
                assert_eq!(B::from(a), b);
            }
        }
        check!(combinations);
    }

    #[test]
    fn multiplication() {
        fn check<T: PauliAssert>() {
            let mapping = [
                (T::I, T::I, T::I),
                (T::I, T::Z, T::Z),
                (T::I, T::X, T::X),
                (T::I, T::Y, T::Y),
                (T::Z, T::I, T::Z),
                (T::Z, T::Z, T::I),
                (T::Z, T::X, T::Y),
                (T::Z, T::Y, T::X),
                (T::X, T::I, T::X),
                (T::X, T::Z, T::Y),
                (T::X, T::X, T::I),
                (T::X, T::Y, T::Z),
                (T::Y, T::I, T::Y),
                (T::Y, T::Z, T::X),
                (T::Y, T::X, T::Z),
                (T::Y, T::Y, T::I),
            ];
            for (mut this, other, expected) in mapping {
                this.multiply(other);
                assert_eq!(this, expected);
            }
        }
        check!();
    }

    #[test]
    fn cliffords() {
        fn check<T: PauliAssert>() {
            #[rustfmt::skip]
            let mapping = [
                //   fn                       fn(I) fn(Z) fn(X) fn(Y)
                (&T::id  as &dyn Fn(&mut T), [T::I, T::Z, T::X, T::Y]),
                (&T::s   as &dyn Fn(&mut T), [T::I, T::Z, T::Y, T::X]),
                (&T::h   as &dyn Fn(&mut T), [T::I, T::X, T::Z, T::Y]),
                (&T::sh  as &dyn Fn(&mut T), [T::I, T::Y, T::Z, T::X]),
                (&T::hs  as &dyn Fn(&mut T), [T::I, T::X, T::Y, T::Z]),
                (&T::shs as &dyn Fn(&mut T), [T::I, T::Y, T::X, T::Z]),
            ];
            for (fun, outputs) in mapping {
                for (expected, mut input) in
                    outputs.into_iter().zip([T::I, T::Z, T::X, T::Y])
                {
                    fun(&mut input);
                    assert_eq!(input, expected)
                }
            }
        }
        check!();
    }

    #[test]
    fn get() {
        fn check<T: PauliAssert>() {
            #[rustfmt::skip]
            let f = [
                // in, get_z, get_x 
                (T::I, false, false),
                (T::Z, true,  false),
                (T::X, false, true),
                (T::Y, true,  true),
            ];
            for (input, get_z, get_x) in f {
                assert_eq!(input.get_z(), get_z);
                assert_eq!(input.get_x(), get_x);
            }
        }
        check!();
    }

    #[test]
    fn set() {
        fn check<T: PauliAssert>() {
            let mapping = [
                // set (false, true) output for input [I, Z, X, Y]
                (
                    &T::set_z as &dyn Fn(&mut T, bool),
                    ([(T::I, T::Z), (T::I, T::Z), (T::X, T::Y), (T::X, T::Y)]),
                ),
                (
                    &T::set_x as &dyn Fn(&mut T, bool),
                    ([(T::I, T::X), (T::Z, T::Y), (T::I, T::X), (T::Z, T::Y)]),
                ),
            ];
            for (fun, outputs) in mapping.into_iter() {
                for ((expected_false, expected_true), mut input) in
                    outputs.into_iter().zip([T::I, T::Z, T::X, T::Y])
                {
                    let mut clone = input;
                    fun(&mut clone, false);
                    assert_eq!(clone, expected_false);
                    fun(&mut input, true);
                    assert_eq!(input, expected_true);
                }
            }
        }
        check!();
    }

    #[test]
    fn partial_add() {
        fn check<T: PauliAssert>() {
            let funs = [
                &T::zpz as &dyn Fn(&mut T, &T),
                &T::zpx as &dyn Fn(&mut T, &T),
                &T::xpz as &dyn Fn(&mut T, &T),
                &T::xpx as &dyn Fn(&mut T, &T),
            ];
            let mapping = [
                //            zpz    zpx   xpz   xpx
                (T::I, T::I, [T::I, T::I, T::I, T::I]),
                (T::I, T::Z, [T::Z, T::I, T::X, T::I]),
                (T::I, T::X, [T::I, T::Z, T::I, T::X]),
                (T::I, T::Y, [T::Z, T::Z, T::X, T::X]),
                (T::Z, T::I, [T::Z, T::Z, T::Z, T::Z]),
                (T::Z, T::Z, [T::I, T::Z, T::Y, T::Z]),
                (T::Z, T::X, [T::Z, T::I, T::Z, T::Y]),
                (T::Z, T::Y, [T::I, T::I, T::Y, T::Y]),
                (T::X, T::I, [T::X, T::X, T::X, T::X]),
                (T::X, T::Z, [T::Y, T::X, T::I, T::X]),
                (T::X, T::X, [T::X, T::Y, T::X, T::I]),
                (T::X, T::Y, [T::Y, T::Y, T::I, T::I]),
                (T::Y, T::I, [T::Y, T::Y, T::Y, T::Y]),
                (T::Y, T::Z, [T::X, T::Y, T::Z, T::Y]),
                (T::Y, T::X, [T::Y, T::X, T::Y, T::Z]),
                (T::Y, T::Y, [T::X, T::X, T::Z, T::Z]),
            ];
            for (this, other, outputs) in mapping {
                for (fun, expected) in funs.iter().zip(outputs) {
                    let mut this = this;
                    fun(&mut this, &other);
                    assert_eq!(this, expected,);
                }
            }
        }
        check!();
    }
}
