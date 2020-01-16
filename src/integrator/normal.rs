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
    fn render(&self, params: RenderParams<T>) -> PixelValue {
        if let Some(hit_record) = params.scene.accel.hit(params.origin) {
            let normal = hit_record.normal;
            return [
                normal.x.to_u8().unwrap(),
                normal.y.to_u8().unwrap(),
                normal.z.to_u8().unwrap(),
            ];
        }
        params.scene.background
    }
}
