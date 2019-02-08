
use std::fs;
use std::io::Read;
use std::io::prelude::*;
use std::error::Error;
use std::path::{PathBuf};

use regex::Regex;
use fs_extra::dir::{get_dir_content2,DirOptions};

lazy_static! {
    pub static ref RE_BUILD_KEY: Regex = Regex::new(r"^([\w|_]+)-([\w]+)$").unwrap();
}

pub fn get_build_keys(target_dir: &PathBuf, build_type: &str) -> Vec<String> {

    let mut fingerprint_dir = target_dir.clone();
    fingerprint_dir.push(build_type);
    fingerprint_dir.push(".fingerprint");

    let mut build_keys = Vec::new();

    let mut dir_options = DirOptions::new();
    dir_options.depth = 0;

    if let Ok(dir_content) = get_dir_content2(&fingerprint_dir, &dir_options) {
        for subdir in dir_content.directories {
            if &subdir == &fingerprint_dir.as_path().to_str().unwrap() {
                //continue;
            }
            let dir = PathBuf::from(subdir);
            let build_key = dir.file_name().unwrap().to_str().unwrap();

            if RE_BUILD_KEY.is_match(build_key) {
                let mut fingerprint_lib_json_file = dir.clone();
                let lib_json_file = format!("lib-{}.json", build_key);
                fingerprint_lib_json_file.push(lib_json_file);

                if fingerprint_lib_json_file.is_file() {
                    build_keys.push(build_key.to_string());
                }
            }
        }
    }

    return build_keys;
}

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
