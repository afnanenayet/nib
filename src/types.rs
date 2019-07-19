//! Defines generic numeric types for the integrator so that operations can be done with generic
//! integers or floating point numbers.

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
macro_rules! combined_trait {
    ( $i:ident; $($t:path),+ ) => {
        pub trait $i: $($t +)+ {}
        impl<T> $i for T where T: $($t +)+ {}
    };
}

combined_trait!(GenInteger; num::NumCast, num::Integer, Copy);
combined_trait!(GenFloat; num::NumCast, num::Float, Copy);
combined_trait!(GenReal; num::NumCast, Copy);

/// The particular floating point type that is going to be used in this program. If you want to
/// switch the float type to another type, simply change the type here.
type Float = f32;

/// The particular unsigned integer type to use in this program. To switch the int type to another
/// type, just change the type here.
type Unsigned = u32;

/// The particular integer type to use in this program.
type Integer = i32;
