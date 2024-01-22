use std::fmt::{self, Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{tableau_encoding, Pauli};

/// Pauli described as enum.
///
/// The discrimants are set according to [tableau_encoding]. Internally, it is very much
/// like [PauliDense](super::dense::PauliDense) (cf. [module](super)).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// annotating it with #[repr(u8)] would allow us to do some unsafe casting (and also
// nice for ffi), but I'm not sure whether this is worth it and I, guess it would limit
// the optimizations the compiler is allowed to perform (there's not much difference in
// the generated assembly code when compiling with --release, but it's still there);
// also, making it repr(u8) is not a breaking change, while vice versa is
pub enum PauliEnum {
    #[default]
    /// Identity
    I = tableau_encoding::I as isize,
    /// Pauli Z
    Z = tableau_encoding::Z as isize,
    /// Pauli X
    X = tableau_encoding::X as isize,
    /// Pauli Y
    Y = tableau_encoding::Y as isize,
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

    fn new_product(z: bool, x: bool) -> Self {
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

impl TryFrom<u8> for PauliEnum {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::I),
            1 => Ok(Self::Z),
            2 => Ok(Self::X),
            3 => Ok(Self::Y),
            _ => Err(value),
        }
    }
}

impl From<PauliEnum> for u8 {
    fn from(value: PauliEnum) -> u8 {
        value.discriminant()
    }
}

impl Display for PauliEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I => write!(f, "I"),
            Self::Z => write!(f, "Z"),
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
        }
    }
}
