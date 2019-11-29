//! A module defining interfaces for an acceleration structure
//!
//! This module provides the generic interface for acceleration structures as well as
//! implementations of various acceleration structures.

use thiserror::Error;

mod list;

pub use list::ObjectList;

#[derive(Error, Debug)]
/// An error associated with acceleration structures
pub enum AccelError {
    #[error("There must be at least one object passed to the constructor")]
    NoObjects,
}

/// A result that can return an `AccelError`
pub type AccelResult<T> = Result<T, AccelError>;
