/*!
Encoding of Pauli operators.
*/

mod single;
pub use single::Pauli;

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
