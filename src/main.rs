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

    println!("Hello world!");
    println!("{:?}", matches);

    let workspace = workspace::Workspace::load(Path::new(PATH));

    /*
    for member in workspace.members() {
        let is_published = cargo::is_published(member);
        println!(" + {} ({}); published = {:?}", member.name(), member.version(), is_published);
    }
    */

    println!("===============");

    let repository = git::Repository::open(Path::new(PATH));

    for member in workspace.members() {
        /*
        if member.has_changelog() {
            member.unpublished(&repository);
        }
        */
    }
}
