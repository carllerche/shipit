use clap::{Arg, App, SubCommand};

use std::path::Path;

const PATH: &str = "/Users/carllerche/Code/Tokio/tokio";
// const PATH: &str = "/Users/carllerche/Code/Tokio/mio";

mod cargo;
mod git;
mod workspace;

fn main() {
    let matches = App::new("Ship It!")
        .version("0.1.0")
        .author("Carl Lerche <me@carllerche.com>")
        .subcommand({
            SubCommand::with_name("check")
                .about("Check for project compliance")
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
        if member.has_changelog() {
            member.unpublished(&repository);
        }
    }
}
