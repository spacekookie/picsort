//! Rename badly numbered files in a directory tree

extern crate clap;

use std::fs::{self, DirEntry};

static mut COUNTER: u32 = 0;
const BASE_PATH: &'static str = "/home/spacekookie/Pictures/Copy/";

/// Run a static callback on a directory tree
///
/// - If a node is a folder, it will recursively execute
/// - If a node is a file, the callback will be called on it
///
/// The provided function **must not** capture an external closure
fn run_on_directory<F: 'static>(path: &str, cb: F, base: &str, ext: &str)
where
    F: Fn(DirEntry, &str, &str) + Clone,
{
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let p = path.unwrap();
        if p.metadata().unwrap().is_dir() {
            run_on_directory(&format!("{}", p.path().display()), cb.clone(), base, ext);
        } else {
            cb(p, base, ext);
        }
    }
}

/// This can't be a closure for some reason :(
fn callback(entry: DirEntry, base: &str, ext: &str) {
    let as_string = &format!("{:?}", entry.file_name());
    let slice = &as_string[1..as_string.len() - 1];

    if slice.starts_with(base) && slice.ends_with(ext) {
        let new = unsafe { 
            COUNTER += 1;
            format!("{}{:>04}.{}", base, COUNTER, ext)
        };
        println!("{} --> {}", slice, new);
        fs::copy(entry.path(), format!("{}{}", BASE_PATH, new)).ok();
    }
}

fn main() {
    let path = "/home/spacekookie/Pictures/Sizilien 2017";
    let base = "DSC_";
    let ext = "NEF";

    run_on_directory(path, callback, base, ext);

    // for path in paths {
    //     let p = path.unwrap();
    //     println!("Name: {}", p.path().display());

    //     if p.metadata().unwrap().is_dir() {
    //         let child = fs::read_dir(format!("{}", p.path().display())).unwrap();
    //         for path in child {
    //             println!("Name: {}", path.unwrap().path().display());
    //         }
    //     }
    // }
}
