//! Defines generic numeric types for the integrator so that operations can be done with generic
//! integers or floating point numbers.

use cgmath::{BaseFloat, BaseNum, Vector3};
use num;
use num_derive::{FromPrimitive, ToPrimitive};
use std::{
    fmt::{Debug, Display},
    slice::Iter,
};

/// Generate a trait that is the sum of other trait bounds
///
/// This is a shortcut to take multiple traits, say `A` and `B`, and create a new trait, `C = A +
/// B`, that implements the other traits automatically.
///
/// For example, `add_traits!(A; B, C)` generates:
/// ```
/// pub trait A: B + C {}
/// impl<T> A for T where T: B + C {}
/// ```
///
/// This macro just helps you avoid the boilerplate.
macro_rules! aggregate_trait {
    ( $i:ident; $($t:path),+ ) => {
        pub trait $i: $($t +)+ {}
        impl<T> $i for T where T: $($t +)+ {}
    };
}

aggregate_trait!(GenReal; BaseNum, Sync, PartialOrd, Copy, Clone, Display, Debug, Send);
aggregate_trait!(GenInteger; num::Integer, GenReal);
aggregate_trait!(GenFloat; BaseFloat, GenReal, num::Signed);

/// The particular floating point type that is going to be used in this program. If you want to
/// switch the float type to another type, simply change the type here.
pub type Float = f32;

/// The particular unsigned integer type to use in this program. To switch the int type to another
/// type, just change the type here.
pub type Unsigned = u32;

/// The particular integer type to use in this program.
pub type Integer = i32;

/// A type representing the RGB value of a pixel in the rendering calculations. This is not the
/// final color value that is output to the buffer.
pub type PixelValue<T> = Vector3<T>;

/// The floating point error threshold to use with the renderer
pub const ETA: Float = 0.000001;

/// The floating point error threshold to use with the renderer, as a convenience function that
/// will automatically convert it to whatever numeric type you want.
pub fn eta<T: GenFloat>() -> T {
    T::from(ETA).unwrap()
}

/// A dimension in 3D space
///
/// This is a type safe way to represent a dimension in the three-dimensional space. Through the
/// use of the `num` derived traits, these dimensions can be converted to indices easily:
///
/// ```
/// # use crate::types::Dimension;
/// use num_traits::ToPrimitive;
///
/// let dimension = Dimension::X;
/// let dim_u32 = dimension.to_u32().unwrap();
/// ```
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Dimension {
    X = 0,
    Y,
    Z,
}

impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Dimension::X => f.write_str("X"),
            Dimension::Y => f.write_str("Y"),
            Dimension::Z => f.write_str("Z"),
        }
    }
}

impl Dimension {
    /// Returns an iterator over the three dimensions
    ///
    /// This method returns an iterator over the elements in the `Dimension` enum
    pub fn iterator() -> Iter<'static, Dimension> {
        use Dimension::*;
        static DIMENSIONS: [Dimension; 3] = [X, Y, Z];
        DIMENSIONS.iter()
    }
}

impl Iterator for Dimension {
    type Item = Dimension;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Dimension::X => Some(Dimension::Y),
            Dimension::Y => Some(Dimension::Z),
            _ => None,
        }
    }
}
