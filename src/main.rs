use clap::{App, Arg, SubCommand};
use shipit::{action, Config, Workspace};
use std::path::Path;

fn main() {
    let matches = App::new("Ship It!")
        .version("0.1.0")
        .author("Carl Lerche <me@carllerche.com>")
        .arg(
            Arg::with_name("project-path")
                .long("project")
                .value_name("path")
                .help("Path of the project root")
                .required(true),
        )
        // .subcommand({ SubCommand::with_name("check").about("Check for project compliance") })
        .subcommand({ SubCommand::with_name("init").about("Initialize a project for shipit") })
        .subcommand({ SubCommand::with_name("status").about("Show the release status") })
        .get_matches();

    let path = matches.value_of("project-path").unwrap();
    let root = Path::new(path);
    let workspace = Workspace::load(root);
    let config = Config::load(&workspace);

    match matches.subcommand() {
        /*
        ("check", Some(_sub_matches)) => {
            action::check::run(&workspace, &config.unwrap().project);
        }
        */
        ("init", Some(_sub_matches)) => {
            let config = match config {
                Ok(config) => Some(config),
                // TODO: better differentiate "not found"
                Err(ref err) if err.downcast_ref::<std::io::Error>().is_some() => None,
                Err(_) => {
                    unimplemented!();
                }
            };

            action::init::run(&workspace, config.as_ref().map(|c| &c.project));
        }
        ("status", Some(_sub_matches)) => {
            let config = config.expect("shipit.toml file missing");
            action::status::run(&workspace, &config);
        }
        _ => {
            unimplemented!();
        }
    }
}
