use pauli_tracker::pauli::Pauli;
use serde::{Deserialize, Serialize};

/// Pauli described as enum.
///
/// The discrimants are set according to [tableau_encoding]. Internally, it is very much
/// like [PauliDense](super::dense::PauliDense) (cf. [module](super)).
#[derive(Clone, Copy, Default, Serialize, Deserialize)] //
#[repr(u8)]
pub enum PauliEnum {
    // using pauli_tracker::pauli::tableau_encoding here makes probles with the output
    // of cbindgen
    #[default]
    /// Identity
    I = 0,
    /// Pauli Z
    Z = 1,
    /// Pauli X
    X = 2,
    /// Pauli Y
    Y = 3,
}

impl PauliEnum {
    /// Get the descriminant of the enum. The discrimant follows [tableau_encoding].
    pub fn discriminant(&self) -> u8 {
        *self as u8
    }
}

macro_rules! const_pauli {
    ($($name:ident,)*) => {$(
        const $name: Self = Self::$name;
    )*};
}

impl Pauli for PauliEnum {
    const_pauli!(I, X, Y, Z,);

    fn new_product(x: bool, z: bool) -> Self {
        match (x, z) {
            (false, false) => Self::I,
            (false, true) => Self::Z,
            (true, false) => Self::X,
            (true, true) => Self::Y,
        }
    }

    fn add(&mut self, other: Self) {
        match (*self, other) {
            (Self::I, _) => *self = other,
            (_, Self::I) => {},
            (Self::X, Self::X) => *self = Self::I,
            (Self::X, Self::Y) => *self = Self::Z,
            (Self::X, Self::Z) => *self = Self::Y,
            (Self::Y, Self::X) => *self = Self::Z,
            (Self::Y, Self::Y) => *self = Self::I,
            (Self::Y, Self::Z) => *self = Self::X,
            (Self::Z, Self::X) => *self = Self::Y,
            (Self::Z, Self::Y) => *self = Self::X,
            (Self::Z, Self::Z) => *self = Self::I,
        }
    }

    fn s(&mut self) {
        match *self {
            Self::I => {},
            Self::X => *self = Self::Y,
            Self::Z => *self = Self::Z,
            Self::Y => *self = Self::X,
        }
    }

    fn h(&mut self) {
        match *self {
            Self::I => {},
            Self::X => *self = Self::Z,
            Self::Z => *self = Self::X,
            Self::Y => *self = Self::Y,
        }
    }

    fn sh(&mut self) {
        match *self {
            Self::I => {},
            Self::X => *self = Self::Z,
            Self::Z => *self = Self::Y,
            Self::Y => *self = Self::X,
        }
    }

    fn hs(&mut self) {
        match *self {
            Self::I => {},
            Self::X => *self = Self::Y,
            Self::Z => *self = Self::X,
            Self::Y => *self = Self::Z,
        }
    }

    fn shs(&mut self) {
        match *self {
            Self::I => {},
            Self::X => *self = Self::X,
            Self::Z => *self = Self::Y,
            Self::Y => *self = Self::Z,
        }
    }

    fn xpx(&mut self, other: &Self) {
        match (*self, *other) {
            (_, Self::I) => {},
            (_, Self::Z) => {},
            (Self::I, Self::X) => *self = Self::X,
            (Self::I, Self::Y) => *self = Self::X,
            (Self::X, Self::X) => *self = Self::I,
            (Self::X, Self::Y) => *self = Self::I,
            (Self::Y, Self::X) => *self = Self::Z,
            (Self::Y, Self::Y) => *self = Self::Z,
            (Self::Z, Self::X) => *self = Self::Y,
            (Self::Z, Self::Y) => *self = Self::Y,
        }
    }

    fn xpz(&mut self, other: &Self) {
        match (*self, *other) {
            (_, Self::I) => {},
            (_, Self::X) => {},
            (Self::I, Self::Z) => *self = Self::X,
            (Self::I, Self::Y) => *self = Self::X,
            (Self::X, Self::Z) => *self = Self::I,
            (Self::X, Self::Y) => *self = Self::I,
            (Self::Y, Self::Z) => *self = Self::Z,
            (Self::Y, Self::Y) => *self = Self::Z,
            (Self::Z, Self::Z) => *self = Self::Y,
            (Self::Z, Self::Y) => *self = Self::Y,
        }
    }

    fn zpx(&mut self, other: &Self) {
        match (*self, *other) {
            (_, Self::I) => {},
            (_, Self::Z) => {},
            (Self::I, Self::X) => *self = Self::Z,
            (Self::I, Self::Y) => *self = Self::Z,
            (Self::X, Self::X) => *self = Self::Y,
            (Self::X, Self::Y) => *self = Self::Y,
            (Self::Y, Self::X) => *self = Self::X,
            (Self::Y, Self::Y) => *self = Self::X,
            (Self::Z, Self::X) => *self = Self::I,
            (Self::Z, Self::Y) => *self = Self::I,
        }
    }

    fn zpz(&mut self, other: &Self) {
        match (*self, *other) {
            (_, Self::I) => {},
            (_, Self::X) => {},
            (Self::I, Self::Z) => *self = Self::Z,
            (Self::I, Self::Y) => *self = Self::Z,
            (Self::X, Self::Z) => *self = Self::Y,
            (Self::X, Self::Y) => *self = Self::Y,
            (Self::Y, Self::Z) => *self = Self::X,
            (Self::Y, Self::Y) => *self = Self::X,
            (Self::Z, Self::Z) => *self = Self::I,
            (Self::Z, Self::Y) => *self = Self::I,
        }
    }

    fn get_x(&self) -> bool {
        match self {
            Self::I => false,
            Self::X => true,
            Self::Y => true,
            Self::Z => false,
        }
    }

    fn get_z(&self) -> bool {
        match self {
            Self::I => false,
            Self::X => false,
            Self::Y => true,
            Self::Z => true,
        }
    }

    fn set_x(&mut self, x: bool) {
        if x {
            match self {
                Self::I => *self = Self::X,
                Self::X => {},
                Self::Y => {},
                Self::Z => *self = Self::Y,
            }
        } else {
            match self {
                Self::I => {},
                Self::X => *self = Self::I,
                Self::Y => *self = Self::Z,
                Self::Z => {},
            }
        }
    }

    fn set_z(&mut self, z: bool) {
        if z {
            match self {
                Self::I => *self = Self::Z,
                Self::X => *self = Self::Y,
                Self::Y => {},
                Self::Z => {},
            }
        } else {
            match self {
                Self::I => {},
                Self::X => {},
                Self::Y => *self = Self::X,
                Self::Z => *self = Self::I,
            }
        }
    }

    fn tableau_encoding(&self) -> u8 {
        self.discriminant()
    }
}
