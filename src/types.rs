//! Defines generic numeric types for the integrator so that operations can be done with generic
//! integers or floating point numbers.

use cgmath::{BaseFloat, BaseNum, Vector3};
use num;
use rand;

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
        impl<T> $i for T where
            rand::distributions::Standard: rand::distributions::Distribution<T>,
            T: $($t +)+,
            {}
    };
}

aggregate_trait!(GenReal; BaseNum, Sync, PartialOrd, Copy, Clone);
aggregate_trait!(GenInteger; num::Integer, GenReal);
aggregate_trait!(GenFloat; BaseFloat, GenReal);

/// The particular floating point type that is going to be used in this program. If you want to
/// switch the float type to another type, simply change the type here.
pub type Float = f32;

/// The particular unsigned integer type to use in this program. To switch the int type to another
/// type, just change the type here.
pub type Unsigned = u32;

/// The particular integer type to use in this program.
pub type Integer = i32;

/// A standard ray with an origin point and a direction
#[derive(Debug, Eq, PartialEq)]
pub struct Ray<T> {
    /// The origin point of the ray in three-dimensional space
    pub origin: Vector3<T>,

    /// The normalized direction of the ray
    ///
    /// The direction of the ray is represented as a normalized 3D vector, which means that every
    /// component of the vector must be between 0 and 1.
    pub direction: Vector3<T>,
}

/// A type representing the RGB value of a pixel with 8-bit channels
pub type PixelValue = [u8; 3];

/// The floating point error threshold to use with the renderer
pub const ETA: f32 = 0.0001;

/// The color black, which is the absence of light
pub const BLACK: [u8; 3] = [0, 0, 0];
