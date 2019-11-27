//! Defines generic numeric types for the integrator so that operations can be done with generic
//! integers or floating point numbers.

use cgmath::Vector3;
use num;

/// Generate a trait that is the sum of other trait bounds
///
/// This is a shortcut to take multiple traits, say `A` and `B`, and create a new trait, `C = A +
/// B`, that implements the other traits automatically.
///
/// For example, `add_traits!(A; B, C)` is the equivalent of doing:
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

aggregate_trait!(GenInteger; num::NumCast, num::Integer, Copy);
aggregate_trait!(GenFloat; num::NumCast, num::Float, Copy);
aggregate_trait!(GenReal; num::NumCast, Copy);

/// The particular floating point type that is going to be used in this program. If you want to
/// switch the float type to another type, simply change the type here.
pub type Float = f32;

/// The particular unsigned integer type to use in this program. To switch the int type to another
/// type, just change the type here.
pub type Unsigned = u32;

/// The particular integer type to use in this program.
pub type Integer = i32;

/// A standard ray with an origin point and a direction
pub struct Ray<T> {
    /// The origin point of the ray in three-dimensional space
    pub origin: Vector3<T>,

    /// The normalized direction of the ray
    ///
    /// The direction of the ray is represented as a normalized 3D vector, which means that every
    /// dimension must be between 0 and 1.
    pub direction: Vector3<T>,
}
