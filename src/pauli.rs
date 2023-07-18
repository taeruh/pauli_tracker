/*!
Encoding of Pauli operators.
*/

macro_rules! new {
    ($(($name:ident, $gate:ident),)*) => {$(
        /// Create a new
        #[doc = stringify!($gate)]
        /// Pauli.
        fn $name() -> Self;
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

#[allow(missing_docs)]
pub trait Pauli {
    /// Create a the new Pauli (X if x) * (Z if z), neglecting phases.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// assert_eq!(PauliDense::new(false, false), PauliDense::new_i());
    /// assert_eq!(PauliDense::new(false, true), PauliDense::new_z());
    /// assert_eq!(PauliDense::new(true, false), PauliDense::new_x());
    /// assert_eq!(PauliDense::new(true, true), PauliDense::new_y());
    /// # }
    fn new(x: bool, z: bool) -> Self;

    new!((new_i, I), (new_x, X), (new_y, Y), (new_z, Z),);

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

macro_rules! new_helper {
    ($(($name:ident, $gate:ident, $const:ident),)*) => {$(
        /// Create a new
        #[doc = stringify!($gate)]
        /// Pauli.
        #[inline]
        fn $name() -> Self {
            $const
        }
    )*};
}
macro_rules! new_impl {
    () => {
        new_helper!(
            (new_i, I, PAULI_I),
            (new_x, X, PAULI_X),
            (new_y, Y, PAULI_Y),
            (new_z, Z, PAULI_Z),
        );
    };
}

pub mod dense;
pub use dense::PauliDense;

pub mod tuple;
pub use tuple::PauliTuple;

impl From<PauliDense> for PauliTuple {
    fn from(pauli: PauliDense) -> Self {
        Self::new(pauli.get_x(), pauli.get_z())
    }
}
impl From<PauliTuple> for PauliDense {
    fn from(pauli: PauliTuple) -> Self {
        Self::new(pauli.get_x(), pauli.get_z())
    }
}

pub mod vec;
pub use vec::PauliVec;

/// Pauli encoding into two bits.
pub mod encoding {
    /// Code for the identity.
    pub const I: u8 = 0;
    /// Code for the Pauli X gate.
    pub const X: u8 = 2;
    /// Code for the Pauli Y gate.
    pub const Y: u8 = 3;
    /// Code for the Pauli Z gate.
    pub const Z: u8 = 1;
}
