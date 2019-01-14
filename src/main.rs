use clap::{App, Arg, SubCommand};

use std::path::Path;

const PATH: &str = "/usr/local/var/foss/tokio";
// const PATH: &str = "/Users/carllerche/Code/Tokio/mio";

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
        .subcommand({ SubCommand::with_name("check").about("Check for project compliance") })
        .subcommand({ SubCommand::with_name("init").about("Initialize a project for shipit") })
        .subcommand({ SubCommand::with_name("status").about("Show the release status") })
        .get_matches();

    // println!("matches == {:?}", &matches);
    println!("11111111111");

    let root = Path::new(PATH);

    println!("22222222222");

    let workspace = Workspace::load(root);

    println!("33333333333");
    let config = Config::load(&workspace);

    println!("44444444444");

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
            println!("KSHDJHDJSAH JSHHSA DJHSAD JHSDKJ");
            action::status::run(&workspace, &config.unwrap());
        }
        _ => {
            unimplemented!();
        }
    }
}
