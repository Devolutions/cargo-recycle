
use clap::App;

use std::path::{PathBuf};

#[derive(Clone)]
pub struct Config {
    pub cargo_home: PathBuf,
    pub recycle_dir: PathBuf,
    pub crate_dir: PathBuf,
    pub export_dir: PathBuf,
}

fn get_cargo_home() -> PathBuf {
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push(".cargo");
    return home_dir;
}

fn get_recyle_dir() -> PathBuf {
    let mut recycle_dir = get_cargo_home();
    recycle_dir.push("recycle");
    return recycle_dir;
}

impl Config {
    pub fn new() -> Self {
        Config {
            cargo_home: get_cargo_home(),
            recycle_dir: get_recyle_dir(),
            crate_dir: PathBuf::new(),
            export_dir: get_recyle_dir(),
        }
    }

    pub fn load_cli(&mut self) {
        let yaml = load_yaml!("cli.yml");
        let app = App::from_yaml(yaml);
        let matches = app.version(crate_version!()).get_matches();

        if let Some(value) = matches.value_of("crate-dir") {
            self.crate_dir = PathBuf::from(value);
        }

        if let Some(value) = matches.value_of("export-dir") {
            self.export_dir = PathBuf::from(value);
        }
    }

    pub fn load() -> Self {
        let mut config = Config::new();
        config.load_cli();
        config
    }
}
