//! An implementation of the Whitted/direct lighting rendering technique. This is the "classic"
//! ray-tracing technique. It was developed by Turner Whitted in 1980, in a paper titled "An
//! Improved Illumination Model for Shaded Display."

use serde::{Deserialize, Serialize};

/// The settings for the Whitted integrator
#[derive(Debug, Serialize, Deserialize)]
pub struct Whitted {
    /// The recursion limit for rays
    ///
    /// This settings sets an upper bound on the depth of the rays in the scene (this is necessary
    /// in case there is infinite recursion in the scene).
    pub u32: max_depth,
}

impl Default for Whitted {
    fn default() -> Self {
        Self { max_depth: 5 }
    }
}
