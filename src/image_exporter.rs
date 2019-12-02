//! Export a framebuffer to some image format
//!
//! Once you have rendered an image, you have a buffer of RGB values. This module provides
//! interfaces to export that framebuffer to a file, such as a PNG or PPM.

use crate::types::PixelValue;
use image::{self, save_buffer_with_format};
use std::{fs::File, io::prelude::*, path::Path};
use thiserror::Error;

/// The possible errors that can arise when exporting a framebuffer
#[derive(Error, Debug)]
pub enum ExporterError {
    #[error("There was some IO error")]
    IO {
        #[from]
        source: std::io::Error,
    },

    #[error("There were invalid pixel values (values were either negative or above the maximum allowed value)")]
    InvalidPixelValues,

    #[error("The supplied width or height were invalid. These values must be greater than 0.")]
    InvalidDimensions,
}

/// A result that can return an `ExporterError`
pub type ExporterResult<T> = Result<T, ExporterError>;

pub trait FramebufferExporter {
    /// Export a buffer of pixel values to a file
    ///
    /// The `buffer` is a vector of RGB pixel values, and the `path` is the desired path to write
    /// the file.
    fn export(&self, buffer: &Vec<PixelValue>, path: &Path) -> ExporterResult<()>;
}

/// Export a framebuffer to the PPM image format
///
/// The PPM format is extremely simple and does not offer any compression. This is a poor choice
/// for large images, as file sizes scale linearly with pixel counts.
#[derive(Debug)]
pub struct PPMExporter {
    /// The width of the output image
    pub width: u32,

    /// The height of the output image
    pub height: u32,
}

impl PPMExporter {
    /// Generate the header for the image format
    fn header(&self) -> ExporterResult<String> {
        if self.width == 0 || self.height == 0 {
            return Err(ExporterError::InvalidDimensions);
        }
        Ok("P3\n".to_owned() + &format!("{} {}\n", self.width, self.height) + "255\n")
    }
}

impl FramebufferExporter for PPMExporter {
    fn export(&self, buffer: &Vec<PixelValue>, path: &Path) -> ExporterResult<()> {
        let header = self.header()?;
        let io_result = {
            let mut file = File::create(path)?;
            file.write(header.as_bytes())?;
            // write each RGB value to the file
            for pixel in buffer {
                let pixel_str = format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]);
                file.write(pixel_str.as_bytes())?;
            }
            Ok(())
        };

        // convert the IO error to an exporter error
        io_result.map_err(|e| ExporterError::IO { source: e })
    }
}

#[derive(Debug)]
pub struct PNGExporter {
    /// The width of the output image
    pub width: u32,

    /// The height of the output image
    pub height: u32,
}

impl FramebufferExporter for PNGExporter {
    fn export(&self, buffer: &Vec<PixelValue>, path: &Path) -> ExporterResult<()> {
        if self.width < 1 || self.height < 1 {
            return Err(ExporterError::InvalidDimensions);
        }
        let flat_buffer: Vec<u8> = buffer.iter().flat_map(|n| n.iter().cloned()).collect();
        image::save_buffer_with_format(
            path,
            &flat_buffer[..],
            self.width,
            self.height,
            image::RGB(8),
            image::PNG,
        )
        .map_err(|e| ExporterError::IO { source: e })
    }
}
