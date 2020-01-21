//! The normal integrator is a diagnostic integrator that simply returns the normal values of the
//! surfaces it hits.
//!
//! This is useful to ensure that normals and surface intersections are being calculated correctly,
//! and that the geometry of the scene is correct.

use crate::{
    integrator::{Integrator, RenderParams},
    types::{GenFloat, PixelValue},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// The parameters for the `Normal` integrator
#[derive(Debug, Serialize, Deserialize)]
pub struct Normal<T: GenFloat> {
    phantom: PhantomData<T>,
}

impl<T: GenFloat> Default for Normal<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: GenFloat> Integrator<T> for Normal<T> {
    fn render(&self, params: RenderParams<T>) -> PixelValue<T> {
        if let Some(accel_record) = params.scene.accel.collision(params.origin) {
            let normal = accel_record.hit_record.normal;
            // Normals can range from -1 to 1, and we need to change that window to [0, 1]. We use
            // the simple formula x' = (0.5 * x) + 0.5
            let x = (normal.x * T::from(0.5).unwrap()) + T::from(0.5).unwrap();
            let y = (normal.y * T::from(0.5).unwrap()) + T::from(0.5).unwrap();
            let z = (normal.z * T::from(0.5).unwrap()) + T::from(0.5).unwrap();
            return PixelValue::new(x, y, z);
        }
        params.scene.background
    }
}
