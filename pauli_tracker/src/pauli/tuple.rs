use std::{
    fmt::{self, Display},
    mem,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Pauli;

/// A Pauli represented by two booleans values. The first one is the X part and the
/// second one is the Z part.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PauliTuple(
    /// Z part
    pub bool,
    /// X part
    pub bool,
);

impl Display for PauliTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::I => write!(f, "I"),
            Self::Z => write!(f, "Z"),
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
        }
    }
}

impl From<(bool, bool)> for PauliTuple {
    fn from(tuple: (bool, bool)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl Pauli for PauliTuple {
    const I: Self = Self(false, false);
    const Z: Self = Self(true, false);
    const Y: Self = Self(true, true);
    const X: Self = Self(false, true);

    fn new_product(z: bool, x: bool) -> Self {
        Self(z, x)
    }

    fn add(&mut self, other: Self) {
        self.0 ^= other.0;
        self.1 ^= other.1;
    }

    fn s(&mut self) {
        self.0 ^= self.1;
    }
    fn h(&mut self) {
        mem::swap(&mut self.1, &mut self.0);
    }
    fn sh(&mut self) {
        // cf. stack impl
        self.h();
        self.s();
    }
    fn hs(&mut self) {
        // cf. stack impl
        self.s();
        self.h();
    }
    fn shs(&mut self) {
        self.1 ^= self.0;
    }

    fn xpx(&mut self, other: &Self) {
        self.1 ^= other.1;
    }

    fn xpz(&mut self, other: &Self) {
        self.1 ^= other.0;
    }

    fn zpx(&mut self, other: &Self) {
        self.0 ^= other.1;
    }

    fn zpz(&mut self, other: &Self) {
        self.0 ^= other.0;
    }

    fn get_x(&self) -> bool {
        self.1
    }

    fn get_z(&self) -> bool {
        self.0
    }

    fn set_x(&mut self, x: bool) {
        self.1 = x;
    }

    fn set_z(&mut self, z: bool) {
        self.0 = z;
    }

    fn tableau_encoding(&self) -> u8 {
        (self.1 as u8) << 1 | self.0 as u8
    }
}
