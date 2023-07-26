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
performed any benchmarks. If needed one can easily create a custom type that implements
[Pauli], so that it can be used for tracking.

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
        #[doc = stringify!($left)]
        /// component onto `self`'s
        #[doc = stringify!($right)]
        /// component in place.
        fn $name(&mut self, other: &Self);
    )*};
}

/// The interface we need for the Pauli tracking
pub trait Pauli {
    const_pauli!(I, X, Y, Z,);
    new_pauli!((new_i, I), (new_x, X), (new_y, Y), (new_z, Z),);

    /// Create a the new Pauli (X if x) * (Z if z), neglecting phases.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// assert_eq!(PauliDense::new_product(false, false), PauliDense::new_i());
    /// assert_eq!(PauliDense::new_product(false, true), PauliDense::new_z());
    /// assert_eq!(PauliDense::new_product(true, false), PauliDense::new_x());
    /// assert_eq!(PauliDense::new_product(true, true), PauliDense::new_y());
    /// # }
    fn new_product(x: bool, z: bool) -> Self;

    /// Add the `other` Pauli to `self` in place.
    fn add(&mut self, other: Self);

    /// Conjugate the Pauli with the Hadamard Gate ignoring phases.
    fn h(&mut self);
    /// Conjugate the Pauli with the S Gate ignoring phases.
    fn s(&mut self);

    plus!((xpx, X, X), (xpz, X, Z), (zpx, Z, X), (zpz, Z, Z),);

    /// Get the Pauli's X component.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let pauli = PauliDense::new_y();
    /// assert_eq!(pauli.get_x(), true);
    /// # }
    fn get_x(&self) -> bool;

    /// Get the Pauli's Z component.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let pauli = PauliDense::new_y();
    /// assert_eq!(pauli.get_z(), true);
    /// # }
    fn get_z(&self) -> bool;

    /// Set whether the Pauli products contains X.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let mut pauli = PauliDense::new_y();
    /// pauli.set_x(false);
    /// assert_eq!(pauli, Pauli::new_z());
    /// # }
    fn set_x(&mut self, x: bool);

    /// Set whether the Pauli products contains Z.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let mut pauli = PauliDense::new_y();
    /// pauli.set_z(false);
    /// assert_eq!(pauli, Pauli::new_x());
    /// # }
    fn set_z(&mut self, z: bool);
}

mod dense;
pub use crate::pauli::dense::PauliDense;

mod enumlike;
pub use enumlike::PauliEnum;

mod tuple;
pub use tuple::PauliTuple;

impl From<PauliEnum> for PauliDense {
    fn from(pauli: PauliEnum) -> Self {
        // panic!();
        unsafe { Self::from_unchecked(pauli.discriminant()) }
    }
}
impl From<PauliTuple> for PauliDense {
    fn from(pauli: PauliTuple) -> Self {
        Self::new_product(pauli.get_x(), pauli.get_z())
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
        Self::new_product(pauli.get_x(), pauli.get_z())
    }
}
impl From<PauliEnum> for PauliTuple {
    fn from(pauli: PauliEnum) -> Self {
        Self::new_product(pauli.get_x(), pauli.get_z())
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
