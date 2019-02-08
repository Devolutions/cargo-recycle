
use std::fs;
use std::io::Read;
use std::io::prelude::*;
use std::error::Error;
use std::path::{PathBuf};

fn load_string_from_file(filename: &str) -> Result<String, Box<Error>> {
    let mut file = fs::File::open(filename).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    Ok(text)
}

fn save_string_to_file(filename: &str, text: &str) {
    let mut file = fs::File::create(filename).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

pub fn replace_in_file(filename: &PathBuf, old: &str, new: &str) {
    let text = load_string_from_file(filename.as_path().to_str().unwrap()).unwrap();
    let text = text.replace(old, new);
    save_string_to_file(filename.as_path().to_str().unwrap(), text.as_str());
}
