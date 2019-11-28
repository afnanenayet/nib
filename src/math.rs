//! Utility math functions  

use crate::types::{GenFloat, GenReal};
use cgmath::Vector3;

/// Calculate the Euclidean distance between two three dimensional vectors
///
/// This method performs the Euclidean distance function on two vectors. Note that you can receive
/// a different type than the type of the input vectors. This is intentional, because the distance
/// between two vectors that are integers, for example, can be a floating point number. Thus, the
/// return value is bounded as a float, but the input vectors can be any numeric type.
pub fn vec3_dist<T: GenReal, R: GenFloat>(a: &Vector3<T>, b: &Vector3<T>) -> R {
    // we need a type `T` and I don't want to type it 3 times
    let two = T::from(2).unwrap();
    let discriminant = (a.x - b.x).powf(two) + (a.y - b.y).powf(two) + (a.z - b.z).powf(two);
    let discriminant = R::from(discriminant).unwrap();
    discriminant.sqrt()
}
