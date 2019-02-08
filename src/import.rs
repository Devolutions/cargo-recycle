
use crate::config::Config;
use crate::file::{get_build_keys,get_script_keys,replace_in_file};

pub fn import_build_unit_fingerprint(config: &Config, build_type: &str, build_key: &str) {
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
    dst_fingerprint_dir.push(build_key);

    fs_extra::dir::create_all(&dst_fingerprint_dir,false).unwrap();
    fs_extra::dir::copy(&src_fingerprint_dir, &dst_fingerprint_dir, &copy_options).unwrap();
}

pub fn import_build_unit_deps(config: &Config, build_type: &str, build_key: &str) {
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;

    let mut src_deps_dir = config.export_dir.clone();
    src_deps_dir.push(build_type);
    src_deps_dir.push("deps");

    let mut dst_deps_dir = config.crate_dir.clone();
    dst_deps_dir.push("target");
    dst_deps_dir.push(build_type);
    dst_deps_dir.push("deps");

    let deps_d_file = format!("{}.d", build_key);
    let deps_rlib_file = format!("lib{}.rlib", build_key);
    let deps_rmeta_file = format!("lib{}.rmeta", build_key);

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

pub fn import_build_unit_build(config: &Config, build_type: &str, build_key: &str) {
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.copy_inside = true;
    copy_options.content_only = true;

    let mut src_build_dir = config.export_dir.clone();
    src_build_dir.push(build_type);
    src_build_dir.push("build");
    src_build_dir.push(build_key);

    if src_build_dir.exists() {
        let mut dst_build_dir = config.crate_dir.clone();
        dst_build_dir.push("target");
        dst_build_dir.push(build_type);
        dst_build_dir.push("build");
        dst_build_dir.push(build_key);

        fs_extra::dir::create_all(&dst_build_dir, false).unwrap();
        fs_extra::dir::copy(src_build_dir, dst_build_dir, &copy_options).unwrap();
    }
}

pub fn import_build_type(config: &Config, build_type: &str) {
    let target_dir = config.export_dir.clone();

    let build_keys = get_build_keys(&target_dir, build_type);

    for build_key in build_keys {
        import_build_unit_fingerprint(config, build_type, &build_key);
        import_build_unit_deps(config, build_type, &build_key);
    }

    let script_keys = get_script_keys(&target_dir, build_type);

    for script_key in script_keys {
        import_build_unit_build(config, build_type, &script_key);
    }
}

pub fn import_rustc_info(config: &Config) {
    let mut src_rustc_info_json = config.export_dir.clone();
    src_rustc_info_json.push(".rustc_info.json");

    let mut dst_target_dir = config.crate_dir.clone();
    dst_target_dir.push("target");

    let mut dst_rustc_info_json = config.crate_dir.clone();
    dst_rustc_info_json.push("target");
    dst_rustc_info_json.push(".rustc_info.json");

    let mut copy_options = fs_extra::file::CopyOptions::new();
    copy_options.overwrite = true;

    fs_extra::dir::create_all(dst_target_dir,false).unwrap();
    fs_extra::file::copy(&src_rustc_info_json, &dst_rustc_info_json, &copy_options).unwrap();
}

pub fn run_import(config: &Config) {
    import_rustc_info(config);
    import_build_type(config, "debug");
    import_build_type(config, "release");
}
