use clap::{Arg, App, SubCommand};

use std::path::Path;

const PATH: &str = "/Users/carllerche/Code/Tokio/tokio";
// const PATH: &str = "/Users/carllerche/Code/Tokio/mio";

mod action;
mod cargo;
mod changelog;
mod config;
mod git;
mod manifest;
mod package;
mod workspace;

use crate::config::Config;
use crate::workspace::Workspace;

fn main() {
    let matches = App::new("Ship It!")
        .version("0.1.0")
        .author("Carl Lerche <me@carllerche.com>")
        .subcommand({
            SubCommand::with_name("check")
                .about("Check for project compliance")
        })
        .subcommand({
            SubCommand::with_name("init")
                .about("Initialize a project for shipit")
        })
        .subcommand({
            SubCommand::with_name("status")
                .about("Show the release status")
        })
        .get_matches();

    println!("matches = {:?}", matches);

    let root = Path::new(PATH);

    let workspace = Workspace::load(root);
    let config = Config::load(&workspace);

    match matches.subcommand() {
        ("check", Some(sub_matches)) => {

            action::check(&workspace, &config);
        }
        ("init", Some(sub_matches)) => {
            unimplemented!();
        }
        ("status", Some(sub_matches)) => {
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    }
}
