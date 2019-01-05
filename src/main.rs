use clap::{Arg, App, SubCommand};
use git2::Repository;

use std::path::Path;

const PATH: &str = "/Users/carllerche/Code/Tokio/tokio";
// const PATH: &str = "/Users/carllerche/Code/Tokio/mio";

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
    println!("{:#?}", workspace);

    /*
    let repo = match Repository::open("/Users/carllerche/Code/Tokio/tokio") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut walk = repo.revwalk().unwrap();
    walk.push_range("tokio-0.1.13..origin/master").unwrap();

    for res in walk {
        println!("{:?}", res.unwrap());
    }
    */
}
