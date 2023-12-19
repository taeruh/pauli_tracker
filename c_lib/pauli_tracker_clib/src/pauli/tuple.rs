use std::mem;

use pauli_tracker::pauli::Pauli;
use serde::{
    Deserialize,
    Serialize,
};

/// A Pauli represented by two booleans values. The first one is the X part and the
/// second one is the Z part.
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct PauliTuple(
    /// X part
    pub bool,
    /// Z part
    pub bool,
);

impl Pauli for PauliTuple {
    const I: Self = Self(false, false);
    const X: Self = Self(true, false);
    const Y: Self = Self(true, true);
    const Z: Self = Self(false, true);

    fn new_product(x: bool, z: bool) -> Self {
        Self(x, z)
    }

    fn add(&mut self, other: Self) {
        self.0 ^= other.0;
        self.1 ^= other.1;
    }

    fn s(&mut self) {
        self.1 ^= self.0;
    }
    fn h(&mut self) {
        mem::swap(&mut self.0, &mut self.1);
    }
    fn sh(&mut self) {
        // cf. stack impl
        mem::swap(&mut self.0, &mut self.1);
        self.1 ^= self.0;
    }
    fn hs(&mut self) {
        // cf. stack impl
        self.1 ^= self.0;
        mem::swap(&mut self.0, &mut self.1);
    }
    fn shs(&mut self) {
        self.0 ^= self.1;
    }

    fn sx(&mut self) {
        self.0 ^= self.1;
    }

    fn xpx(&mut self, other: &Self) {
        self.0 ^= other.0;
    }

    fn xpz(&mut self, other: &Self) {
        self.0 ^= other.1;
    }

    fn zpx(&mut self, other: &Self) {
        self.1 ^= other.0;
    }

    fn zpz(&mut self, other: &Self) {
        self.1 ^= other.1;
    }

    fn get_x(&self) -> bool {
        self.0
    }

    fn get_z(&self) -> bool {
        self.1
    }

    fn set_x(&mut self, x: bool) {
        self.0 = x;
    }

    fn set_z(&mut self, z: bool) {
        self.1 = z;
    }

    fn tableau_encoding(&self) -> u8 {
        (self.0 as u8) << 1 | self.1 as u8
    }
}
