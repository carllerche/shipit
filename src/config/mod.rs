pub mod error;
mod project;
mod system;
mod tag;

pub use self::project::{Package, Project};
pub use self::system::System;
pub use self::tag::TagFormat;

use crate::{Error, Workspace};

#[derive(Debug)]
pub struct Config {
    /// Project level configuration
    pub project: Project,

    /// System level configuration
    pub system: System,
}

impl Config {
    pub fn load(workspace: &Workspace) -> Result<Config, Error> {
        Ok(Config {
            project: Project::load(workspace)?,
            system: System::load(),
        })
    }
}
