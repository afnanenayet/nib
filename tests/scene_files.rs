//! This module tests all of the example scene files to ensure that documentation and examples are
//! up to date. We want to ensure that the configuration files can be parsed correctly.

use std::{fs, path::PathBuf};

/// Get a list of all of the scene files in the given directory
fn list_scene_files(dir: PathBuf) -> Vec<PathBuf> {
    let scene_files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    scene_files
}

#[test]
fn example_scene_files_parse() {
    todo!();
}
