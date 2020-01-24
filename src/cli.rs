//! This module handles everything related to the CLI interface, such as arguments, "UI", and
//! control flow.

use crate::scene::*;
use anyhow::{self, format_err};
use json5;
use ron;
use serde_yaml;
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

/// An oxidized renderer
#[derive(StructOpt, Debug)]
#[structopt(author = "Afnan Enayet")]
pub struct Args {
    /// The path to the file describing the scene
    pub scene: PathBuf,

    /// The file type of the scene description file. If this is not supplied, the application will
    /// attempt to guess the file type from the file extension. Valid values are: "ron", "yaml",
    /// "yml", "json".
    #[structopt(short, long)]
    pub filetype: Option<String>,

    /// Only parse the scene file and do nothing else. This can be helpful when trying to construct
    /// scene files to ensure that they conform to the spec and will be deserialized properly.
    #[structopt(short = "r", long)]
    pub only_parse: bool,

    /// If enabled, this flag will hide the progress bar. The progress bar is ordinarily displayed
    /// to STDERR.
    #[structopt(short = "p", long = "hide-progress")]
    pub hide_progress: bool,

    /// The number of threads to use in the renderer. If this isn't set, the renderer will default
    /// to the number of CPUs detected.
    #[structopt(short, long)]
    pub threads: Option<u32>,

    /// The filename of the output file. If this is not provided it will default to "out.png". The
    /// output file type is inferred from the filename.
    #[structopt(short, long)]
    pub output: Option<String>,

    /// The vertical resolution of the output image
    #[structopt(short, long)]
    pub height: u32,

    /// The horizontal resolution of the output image
    #[structopt(short, long)]
    pub width: u32,
}

/// Parse the input scene file based on the file extension
///
/// Because I am allowing multiple types of files for the scene files (e.g. JSON or YAML), and also
/// may change the formats in the future, we have a wrapper method to parse the scene file into a
/// scene struct. Right now it attempts to determine the file type from the path's extension.
///
/// Right now we support the following filetypes for the scene description:
/// - yaml
/// - json
/// - json5 (which is a superset of json)
/// - RON (Rusty Object Notation)
///
/// I would recommend using RON since it's the most expressive give that we are using Rust data
/// structures, and it has full support for all of serde's data types, which is what we're using to
/// serialize.
pub fn dispatch_scene_parse(path: &PathBuf, ext: Option<&str>) -> anyhow::Result<Scene<f32>> {
    if !path.exists() {
        return Err(format_err!(
            "Path to scene file \"{}\" does not exist",
            path.to_string_lossy()
        ));
    }
    let mut file_str = String::new();
    File::open(path)?.read_to_string(&mut file_str)?;
    let candidate_ext: Option<&str> = match ext {
        None => path.extension().map(|x| x.to_str().unwrap_or_default()),
        Some(x) => Some(x),
    };

    match candidate_ext {
        None => Err(format_err!(
            "Could not determine the filetype of the scene file"
        )),
        Some(ext) => match ext {
            "ron" => ron::de::from_str(&file_str).map_err(|x| x.into()),
            "json" => json5::from_str(&file_str).map_err(|x| x.into()),
            "yaml" | "yml" => serde_yaml::from_str(&file_str).map_err(|x| x.into()),
            _ => Err(format_err!("Filetype \"{}\" is not supported", ext)),
        },
    }
}
