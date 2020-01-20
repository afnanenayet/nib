//! This module handles everything related to the CLI interface, such as arguments, "UI", and
//! control flow.

use crate::scene::*;
use anyhow::{self, format_err};
use serde_json;
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

/// An oxidized renderer
#[derive(StructOpt, Debug)]
#[structopt(author = "Afnan Enayet")]
pub struct Args {
    /// The path to the file describing the scene
    pub scene: PathBuf,

    /// If enabled, this flag will hide the progress bar. The progress bar is ordinarily displayed
    /// to STDERR.
    #[structopt(short = "p", long = "hide-progress")]
    pub hide_progress: bool,

    /// The number of threads to use in the renderer. If this isn't set, the renderer will default
    /// to the number of CPUs detected.
    #[structopt(short, long)]
    pub threads: Option<u32>,
}

/// Parse the input scene file based on the file extension
///
/// Because I am allowing multiple types of files for the scene files (e.g. JSON or YAML), and also
/// may change the formats in the future, we have a wrapper method to parse the scene file into a
/// scene struct. Right now it attempts to determine the file type from the path's extension.
pub fn dispatch_scene_parse(path: &PathBuf) -> anyhow::Result<Scene<f32>> {
    if !path.exists() {
        return Err(format_err!(
            "Path to scene file \"{}\" does not exist",
            path.to_string_lossy()
        ));
    }

    match path.extension() {
        None => Err(format_err!(
            "Could not determine the filetype of the scene file"
        )),
        Some(ext) => {
            let ext = ext.to_str().unwrap_or_default();
            let mut scene_file = File::open(path)?;
            let mut buffer = String::new();
            scene_file.read_to_string(&mut buffer)?;
            match ext {
                "json" => serde_json::from_str(&buffer).map_err(|x| x.into()),
                _ => Err(format_err!(
                    "Could not determine the filetype of the scene file"
                )),
            }
        }
    }
}
