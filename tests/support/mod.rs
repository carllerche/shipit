#![allow(dead_code)]

pub mod fixture;
pub mod git;
// pub mod github;

use std::path::Path;

/// Read the contents of a file
pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = match File::open(path.as_ref()) {
        Ok(f) => f,
        Err(e) => panic!(
            "failed to open file; path = {:?}; err={:?}",
            path.as_ref(),
            e
        ),
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}
