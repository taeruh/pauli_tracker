use std::mem;

use super::Pauli;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct PauliTuple(bool, bool);

macro_rules! const_pauli {
    ($(($name:ident, $x:literal, $z:literal),)*) => {$(
        const $name: Self = Self ($x, $z);
    )*};
}

impl Pauli for PauliTuple {
    const_pauli!(
        (I, false, false),
        (X, true, false),
        (Y, true, true),
        (Z, false, true),
    );

    fn new(x: bool, z: bool) -> Self {
        Self(x, z)
    }

    new_impl!();

    #[inline]
    fn add(&mut self, other: Self) {
        self.0 ^= other.0;
        self.1 ^= other.1;
    }

    fn h(&mut self) {
        mem::swap(&mut self.0, &mut self.1);
    }

    #[inline]
    fn s(&mut self) {
        self.1 ^= self.0;
    }

    #[inline]
    fn xpx(&mut self, other: &Self) {
        self.0 ^= other.0;
    }

    #[inline]
    fn xpz(&mut self, other: &Self) {
        self.0 ^= other.1;
    }

    #[inline]
    fn zpx(&mut self, other: &Self) {
        self.1 ^= other.0;
    }

    #[inline]
    fn zpz(&mut self, other: &Self) {
        self.1 ^= other.1;
    }

    #[inline]
    fn get_x(&self) -> bool {
        self.0
    }

    #[inline]
    fn get_z(&self) -> bool {
        self.1
    }

    fn set_x(&mut self, x: bool) {
        self.0 = x;
    }

    fn set_z(&mut self, z: bool) {
        self.1 = z;
    }
}