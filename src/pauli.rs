/*!
Encoding of Pauli operators.
*/

mod single;
pub use single::Pauli;

mod vec;
pub use vec::PauliVec;

/// Code for the identity.
pub(crate) const I: u8 = 0;
/// Code for the Pauli X gate.
pub(crate) const X: u8 = 2;
/// Code for the Pauli Y gate.
pub(crate) const Y: u8 = 3;
/// Code for the Pauli Z gate.
pub(crate) const Z: u8 = 1;
