use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Write};


pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file =OpenOptions::new().write(true).create(true).open(path);

   let _ = file.unwrap().write_all(content.as_bytes());
}