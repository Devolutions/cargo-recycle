
use crate::config::Config;
use crate::file::{get_build_keys,replace_in_file,RE_BUILD_KEY};

pub fn import_build_unit(config: &Config, build_type: &str, build_name: &str, build_meta: &str) {
    let build_key = format!("{}-{}", build_name, build_meta);

    // import .fingerprint directory artifacts

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.copy_inside = true;
    copy_options.content_only = true;

    let mut src_fingerprint_dir = config.export_dir.clone();
    src_fingerprint_dir.push(build_type);
    src_fingerprint_dir.push(".fingerprint");

    let mut dst_fingerprint_dir = config.crate_dir.clone();
    dst_fingerprint_dir.push("target");
    dst_fingerprint_dir.push(build_type);
    dst_fingerprint_dir.push(".fingerprint");
    dst_fingerprint_dir.push(build_key.as_str());

    fs_extra::dir::create_all(&dst_fingerprint_dir,false).unwrap();
    fs_extra::dir::copy(&src_fingerprint_dir, &dst_fingerprint_dir, &copy_options).unwrap();

    // export deps directory artifacts

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;

    let mut src_deps_dir = config.export_dir.clone();
    src_deps_dir.push(build_type);
    src_deps_dir.push("deps");

    let mut dst_deps_dir = config.crate_dir.clone();
    dst_deps_dir.push("target");
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

    fs_extra::dir::create_all(&dst_deps_dir,false).unwrap();
    fs_extra::copy_items(&src_files, &dst_deps_dir, &copy_options).unwrap();

    // modify exported files

    replace_in_file(&dst_deps_d_file, "$CARGO_HOME", config.cargo_home.to_str().unwrap());
    replace_in_file(&dst_deps_d_file, "$CRATE_DIR", config.crate_dir.to_str().unwrap());
}

pub fn import_build_type(config: &Config, build_type: &str) {
    let target_dir = config.export_dir.clone();

    let build_keys = get_build_keys(&target_dir, build_type);

    for build_key in build_keys {
        if let Some(captures) = RE_BUILD_KEY.captures(build_key.as_str()) {
            let build_name = &captures[1];
            let build_meta = &captures[2];
            import_build_unit(config, build_type, &build_name, &build_meta);
        }
    }
}

pub fn import_rustc_info(config: &Config) {
    let mut src_rustc_info_json = config.export_dir.clone();
    src_rustc_info_json.push(".rustc_info.json");

    let mut dst_rustc_info_json = config.crate_dir.clone();
    dst_rustc_info_json.push("target");
    dst_rustc_info_json.push(".rustc_info.json");

    let mut copy_options = fs_extra::file::CopyOptions::new();
    copy_options.overwrite = true;

    fs_extra::dir::create_all(&config.export_dir,false).unwrap();
    fs_extra::file::copy(&src_rustc_info_json, &dst_rustc_info_json, &copy_options).unwrap();
}

pub fn run_import(config: &Config) {
    import_rustc_info(config);
    import_build_type(config, "debug");
    import_build_type(config, "release");
}
