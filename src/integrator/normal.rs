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
            return PixelValue::new(normal.x, normal.y, normal.z);
        }
        params.scene.background
    }
}
