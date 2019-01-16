mod project;
mod system;

pub use self::project::{Project, Package, TagFormat};
pub use self::system::System;

use crate::Workspace;

pub struct Config {
    /// Project level configuration
    pub project: Project,

    /// System level configuration
    pub system: System,
}

#[derive(Debug)]
pub enum LoadError {
    NotFound,
}

impl Config {
    pub fn load(workspace: &Workspace) -> Result<Config, LoadError> {
        Ok(Config {
            project: Project::load(workspace)?,
            system: System::load(),
        })
    }
}
