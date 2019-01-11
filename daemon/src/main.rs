extern crate flubber;
extern crate structopt;
extern crate tokio;
extern crate toml;
extern crate xdg;

use std::fs;

use flubber::{Config, Flubber};
use structopt::StructOpt;
use tokio::prelude::Future;
use xdg::BaseDirectories;

#[derive(Debug, StructOpt)]
struct Opt {}

fn main() {
    // TODO: parse config options from this
    let _ = Opt::from_args();

    // TODO: don't unwrap
    let base_dir = BaseDirectories::new().unwrap();
    let config_file = base_dir.find_config_file("flubber.conf");
    let config = match config_file {
        Some(path) => {
            let contents = fs::read(path).unwrap();
            toml::from_slice::<Config>(&contents).unwrap()
        }
        None => {
            let path = base_dir.place_config_file("flubber.conf").unwrap();
            let config = Config::default();
            println!("First run, writing config file.");
            let contents = toml::to_string_pretty(&config).unwrap();
            fs::write(path, contents.as_bytes()).unwrap();
            config
        }
    };

    let flubber = Flubber::from_config(config);
    tokio::run(flubber.as_future().map_err(|err| {
        eprintln!("Failed with error: {}", err);
        ()
    }));
}
