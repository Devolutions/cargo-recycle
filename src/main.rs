
#[macro_use]
extern crate clap;

extern crate dirs;
extern crate fs_extra;

use clap::App;

use std::error::Error;

#[macro_use]
extern crate lazy_static;

mod config;
mod export;
mod error;
mod file;

use crate::config::Config;
use crate::export::run_export;

fn run(matches: &clap::ArgMatches, config: &Config) -> Result<(), Box<Error>> {
    match matches.subcommand() {
        ("import", Some(_args)) => {
            println!("import!");
            Ok(())
        },
        ("export", Some(_args)) => {
            println!("export!");
            run_export(config);
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
