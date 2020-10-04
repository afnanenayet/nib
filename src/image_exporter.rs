//! Export a framebuffer to some image format
//!
//! Once you have rendered an image, you have a buffer of RGB values. This module provides
//! interfaces to export that framebuffer to a file, such as a PNG or PPM.

use crate::types::{Float, PixelValue};
use image::{self, save_buffer_with_format};
use num::traits::*;
use std::{fs::File, io::prelude::*, path::Path};
use thiserror::Error;

/// An enum type describing the possible output filetypes for the resulting image
#[derive(Debug, PartialEq, Eq)]
pub enum OuputType {
    PNG,
    PPM,
}

/// The possible errors that can arise when exporting a framebuffer
#[derive(Error, Debug)]
pub enum ExporterError {
    #[error("There was some error from the image library")]
    Image {
        #[from]
        source: image::ImageError,
    },

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

/// The "base" trait for a `FrameBufferExporter`
///
/// Implementing this trait automatically implements the `FrameBufferExporter` trait, which
/// provides some methods for free, such as converting the values from values between 0 and 1 to
/// N-bit color values.
pub trait FramebufferExporterBase {
    /// Export a buffer of pixel values to a file
    ///
    /// The `buffer` is a vector of RGB pixel values, and the `path` is the desired path to write
    /// the file. You can assume that the buffer will consist of integers between 0 and
    /// `MAX_COLOR`.
    fn export(&self, buffer: &[PixelValue<u32>], path: &Path) -> ExporterResult<()>;

    /// The maximum color value that a framebuffer
    const MAX_COLOR: u32;
}

/// Something that can export a framebuffer of `PixelValue`s to some other format
///
/// Users should use this trait, implementors should use the `FramebufferExporterBase` trait, which
/// abstracts away some common methods, such as converting a floating point value to an integer RGB
/// value.
pub trait FramebufferExporter {
    /// Export a buffer of floating point pixel values to some other format
    ///
    /// This method expects a framebuffer of pixel values between 0 and 1, that haven't been
    /// converted to some specific color format yet.
    fn export(&self, buffer: &[PixelValue<Float>], path: &Path) -> ExporterResult<()>;
}

impl<T: FramebufferExporterBase> FramebufferExporter for T {
    fn export(&self, buffer: &[PixelValue<Float>], path: &Path) -> ExporterResult<()> {
        // Convert the floating point color values to proper N-bit integer color values, based on
        // the `MAX_COLOR` value
        let int_buffer: Vec<PixelValue<u32>> = buffer
            .iter()
            .map(|pixel| {
                let max_value = num::NumCast::from(T::MAX_COLOR).unwrap();
                (pixel * max_value).map(|x| x.to_u32().unwrap())
            })
            .collect();
        self.export(&int_buffer[..], path)
    }
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

impl FramebufferExporterBase for PPMExporter {
    const MAX_COLOR: u32 = 255;

    fn export(&self, buffer: &[PixelValue<u32>], path: &Path) -> ExporterResult<()> {
        let header = self.header()?;

        let io_result = {
            let mut file = File::create(path)?;
            file.write(header.as_bytes())?;
            // write each RGB value to the file

            for pixel in buffer {
                let pixel_str = format!("{} {} {}\n", pixel.x, pixel.y, pixel.z);
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

impl FramebufferExporterBase for PNGExporter {
    const MAX_COLOR: u32 = 255;

    fn export(&self, buffer: &[PixelValue<u32>], path: &Path) -> ExporterResult<()> {
        if self.width < 1 || self.height < 1 {
            return Err(ExporterError::InvalidDimensions);
        }
        // We need to flatten our vector of (mathematical) vectors into a `Vec` of 8 bit color
        // values. We convert the vector values into `Vec` types so we can iterate over the pixels.
        // This lets us leverage Rust's built-in method to flatten iterators of iterators. The
        // cgmath vector type does not offer an iterator, unfortunately.

        // TODO(afnan) consider forking cgmath and adding the iterators to avoid this layer of
        // indirection, which incurs some extra allocation.
        let u8_buffer = buffer
            .iter()
            // We're doing some shenanigans to convert a range [0, 1] to [0, 255], which you can
            // also interpret as converting a float to an 8-bit integer.
            .map(|v| {
                vec![
                    v.x.to_u8().unwrap(),
                    v.y.to_u8().unwrap(),
                    v.z.to_u8().unwrap(),
                ]
            })
            .collect::<Vec<Vec<u8>>>();

        // We need to flatten the buffer in another step, because we lose the temporary vector if
        // we try to flatten out the structure in one go
        let flat_buffer = u8_buffer
            .iter()
            .flatten()
            .map(|x| x.clone())
            .collect::<Vec<u8>>();

        save_buffer_with_format(
            path,
            &flat_buffer[..],
            self.width,
            self.height,
            image::ColorType::Rgb8,
            image::ImageFormat::Png,
        )
        .map_err(|e| ExporterError::Image { source: e })
    }
}
