
use std::path::{PathBuf};

use crate::config::Config;
use crate::file::replace_in_file;

use regex::Regex;
use fs_extra::dir::{get_dir_content2,DirOptions};

lazy_static! {
    static ref RE_BUILD_KEY: Regex = Regex::new(r"^([\w|_]+)-([\w]+)$").unwrap();
}

fn get_build_keys(config: &Config, build_type: &str) -> Vec<String> {

    let mut fingerprint_dir = config.crate_dir.clone();
    fingerprint_dir.push("target");
    fingerprint_dir.push(build_type);
    fingerprint_dir.push(".fingerprint");

    let mut build_keys = Vec::new();

    let mut dir_options = DirOptions::new();
    dir_options.depth = 1;

    if let Ok(dir_content) = get_dir_content2(&fingerprint_dir, &dir_options) {
        for subdir in dir_content.directories {
            if &subdir == &fingerprint_dir.as_path().to_str().unwrap() {
                continue;
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

pub fn export_build_unit(config: &Config, build_type: &str, package_name: &str, package_meta: &str) {
    let build_key = format!("{}-{}", package_name, package_meta);

    // export .fingerprint directory artifacts

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.copy_inside = true;
    copy_options.content_only = true;

    let mut src_fingerprint_dir = config.crate_dir.clone();
    src_fingerprint_dir.push("target");
    src_fingerprint_dir.push(build_type);
    src_fingerprint_dir.push(".fingerprint");
    src_fingerprint_dir.push(build_key.as_str());

    let mut dst_fingerprint_dir = config.export_dir.clone();
    dst_fingerprint_dir.push(build_type);
    dst_fingerprint_dir.push(".fingerprint");

    fs_extra::dir::copy(src_fingerprint_dir, dst_fingerprint_dir, &copy_options).unwrap();

    // export deps directory artifacts

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;

    let mut src_deps_dir = config.crate_dir.clone();
    src_deps_dir.push("target");
    src_deps_dir.push(build_type);
    src_deps_dir.push("deps");

    let mut dst_deps_dir = config.export_dir.clone();
    dst_deps_dir.push(build_type);
    dst_deps_dir.push("deps");

    let deps_d_file = format!("{}.d", build_key.as_str());
    let deps_rlib_file = format!("lib{}.rlib", build_key.as_str());
    let deps_rmeta_file = format!("lib{}.rmeta", build_key.as_str());

    // deps src files

    let mut src_deps_d_file = src_deps_dir.clone();
    src_deps_d_file.push(deps_d_file.as_str());

    let mut src_deps_rlib_file = src_deps_dir.clone();
    src_deps_rlib_file.push(deps_rlib_file.as_str());

    let mut src_deps_rmeta_file = src_deps_dir.clone();
    src_deps_rmeta_file.push(deps_rmeta_file.as_str());

    let mut src_files = Vec::new();
    src_files.push(&src_deps_d_file);

    if src_deps_rlib_file.exists() {
        src_files.push(&src_deps_rlib_file);
    }

    if src_deps_rmeta_file.exists() {
        src_files.push(&src_deps_rmeta_file);
    }

    // deps dst files

    let mut dst_deps_d_file = dst_deps_dir.clone();
    dst_deps_d_file.push(&deps_d_file);

    let mut dst_deps_rlib_file = dst_deps_dir.clone();
    dst_deps_rlib_file.push(&deps_rlib_file);

    fs_extra::dir::create_all(&dst_deps_dir,false).unwrap();
    fs_extra::copy_items(&src_files, &dst_deps_dir, &copy_options).unwrap();

    // modify exported files

    replace_in_file(&dst_deps_d_file, config.cargo_home.to_str().unwrap(), "$CARGO_HOME");
    replace_in_file(&dst_deps_d_file, config.crate_dir.to_str().unwrap(), "$CRATE_DIR");
}

pub fn export_build_type(config: &Config, build_type: &str) {
    let build_keys = get_build_keys(config, build_type);

    for build_key in build_keys {
        if let Some(captures) = RE_BUILD_KEY.captures(build_key.as_str()) {
            let build_name = &captures[1];
            let build_meta = &captures[2];
            export_build_unit(config, build_type, &build_name, &build_meta);
        }
    }
}

pub fn run_export(config: &Config) {
    export_build_type(config, "debug");
    export_build_type(config, "release");
}
