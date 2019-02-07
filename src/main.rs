
#[macro_use]
extern crate clap;

extern crate dirs;
extern crate fs_extra;

use clap::App;

use std::fs;
use std::error::Error;
use std::io::Read;
use std::io::prelude::*;
use std::path::{PathBuf};

mod config;
use crate::config::Config;

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

fn replace_in_file(filename: &PathBuf, old: &str, new: &str) {
    let text = load_string_from_file(filename.as_path().to_str().unwrap()).unwrap();
    let text = text.replace(old, new);
    save_string_to_file(filename.as_path().to_str().unwrap(), text.as_str());
}

fn run_export(config: &Config, build_type: &str, package_name: &str, package_meta: &str) {

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

    // deps src files

    let mut src_deps_d_file = src_deps_dir.clone();
    src_deps_d_file.push(deps_d_file.as_str());

    let mut src_deps_rlib_file = src_deps_dir.clone();
    src_deps_rlib_file.push(deps_rlib_file.as_str());

    let mut src_files = Vec::new();
    src_files.push(&src_deps_d_file);
    src_files.push(&src_deps_rlib_file);

    // deps dst files

    let mut dst_deps_d_file = dst_deps_dir.clone();
    dst_deps_d_file.push(&deps_d_file);

    let mut dst_deps_rlib_file = dst_deps_dir.clone();
    dst_deps_rlib_file.push(&deps_rlib_file);

    let mut dst_files = Vec::new();
    dst_files.push(&dst_deps_d_file);
    dst_files.push(&dst_deps_rlib_file);

    fs_extra::dir::create_all(&dst_deps_dir,false).unwrap();
    fs_extra::copy_items(&src_files, &dst_deps_dir, &copy_options).unwrap();

    // modify exported files

    replace_in_file(&dst_deps_d_file, config.cargo_home.to_str().unwrap(), "$CARGO_HOME");
    replace_in_file(&dst_deps_d_file, config.crate_dir.to_str().unwrap(), "$CRATE_DIR");
}

fn run(matches: &clap::ArgMatches, config: &Config) -> Result<(), Box<Error>> {
    match matches.subcommand() {
        ("import", Some(_args)) => {
            println!("import!");
            Ok(())
        },
        ("export", Some(_args)) => {
            println!("export!");
            let build_type = "debug";
            let package_name = "lazy_static";
            let package_meta = "57da46e343ce1b38";
            run_export(config, &build_type, &package_name, &package_meta);
            Ok(())
        }
        _ => { Err("unrecognized command")? },
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.version(crate_version!()).get_matches();

    let config = Config::load();
    run(&matches, &config).unwrap();
}
