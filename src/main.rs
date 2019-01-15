use clap::{App, Arg, SubCommand};

use std::path::Path;

mod action;
mod cargo;
mod changelog;
mod config;
mod git;
mod github;
mod manifest;
mod package;
mod workspace;

use crate::config::Config;
use crate::workspace::Workspace;

fn main() {
    let matches = App::new("Ship It!")
        .version("0.1.0")
        .author("Carl Lerche <me@carllerche.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Path of the target project")
                .required(true)
                .index(1),
        )
        .subcommand({ SubCommand::with_name("check").about("Check for project compliance") })
        .subcommand({ SubCommand::with_name("init").about("Initialize a project for shipit") })
        .subcommand({ SubCommand::with_name("status").about("Show the release status") })
        .get_matches();

    let path = matches.value_of("INPUT").unwrap();
    let root = Path::new(path);
    let workspace = Workspace::load(root);
    let config = Config::load(&workspace);

    match matches.subcommand() {
        ("check", Some(sub_matches)) => {
            action::check::run(&workspace, &config.unwrap());
        }
        ("init", Some(sub_matches)) => {
            let config = match config {
                Ok(config) => Some(config),
                Err(ref err) if err.is_not_found() => None,
                Err(_) => {
                    unimplemented!();
                }
            };

            action::init::run(&workspace, config.as_ref());
        }
        ("status", Some(sub_matches)) => {
            action::status::run(&workspace, &config.unwrap());
        }
        _ => {
            unimplemented!();
        }
    }
}
