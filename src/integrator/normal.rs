//! The normal integrator is a diagnostic integrator that simply returns the normal values of the
//! surfaces it hits.
//!
//! This is useful to ensure that normals and surface intersections are being calculated correctly,
//! and that the geometry of the scene is correct.

use crate::{
    integrator::{Integrator, RenderParams},
    types::{Float, PixelValue},
};
use serde::{Deserialize, Serialize};

/// The parameters for the `Normal` integrator
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Normal {}

impl Integrator for Normal {
    fn render(&self, params: RenderParams) -> PixelValue<Float> {
        if let Some(accel_record) = params.context.accel.collision(params.origin) {
            let normal = accel_record.hit_record.normal;
            // Normals can range from -1 to 1, and we need to change that window to [0, 1]. We use
            // the simple formula x' = (0.5 * x) + 0.5
            let x = (normal.x * 0.5) + 0.5;
            let y = (normal.y * 0.5) + 0.5;
            let z = (normal.z * 0.5) + 0.5;
            return PixelValue::new(x, y, z);
        }
        params.context.background
    }
}
