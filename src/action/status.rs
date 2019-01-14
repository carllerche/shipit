use crate::cargo;
use crate::git;
use crate::github;
use crate::package;
use crate::{Config, Workspace};

use semver::Version;

use slog::*;

/**
WORKSPACE STRUCTURE
{
    members:
        [
        Package {
            name: "tokio",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 14,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio"
        },
        Package {
            name: "tokio-async-await",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 5,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-async-await"
        },
        Package {
            name: "tokio-buf",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-buf"
        },
        Package {
            name: "tokio-channel",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-channel"
        },
        Package {
            name: "tokio-codec",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 1,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-codec"
        },
        Package {
            name: "tokio-current-thread",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 4,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-current-thread"
        },
        Package {
            name: "tokio-executor",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 6,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-executor"
        },
        Package {
            name: "tokio-fs",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 5,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-fs"
        },
        Package {
            name: "tokio-io",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 11,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-io"
        },
        Package {
            name: "tokio-reactor",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 8,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-reactor"
        },
        Package {
            name: "tokio-signal",
            manifest_version: Version {
                major: 0,
                minor: 2,
                patch: 7,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-signal"
        },
        Package {
            name: "tokio-threadpool",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 10,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-threadpool"
        },
        Package {
            name: "tokio-timer",
            manifest_version: Version {
                major: 0,
                minor: 2,
                patch: 8,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-timer"
        },
        Package {
            name: "tokio-tcp",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 3,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-tcp"
        },
        Package {
            name: "tokio-tls",
            manifest_version: Version {
                major: 0,
                minor: 2,
                patch: 1,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-tls"
        },
        Package {
            name: "tokio-udp",
            manifest_version: Version {
                major: 0,
                minor: 1,
                patch: 3,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-udp"
        },
        Package {
            name: "tokio-uds",
            manifest_version: Version {
                major: 0,
                minor: 2,
                patch: 5,
                pre: [],
                build: []
            },
            path: "/usr/local/var/foss/tokio/tokio-uds"
        }],
         root: "/usr/local/var/foss/tokio" }

*/

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &Config) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    let github = github::Client::new();
    let pulls = github.prs();

    for pr in pulls.iter() {
        info!(log, "{:?}\n", pr);
    }

    let repository = git::Repository::open(workspace.root());

    let zero_one_zero = Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: vec![],
        build: vec![],
    };

    for member in workspace.members() {
        let config = &config.packages[member.name()];

        let mut published = cargo::published_versions(member.name());

        if let Some(init_version) = config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();
        if let Some(tag_format) = config.tag_format {
            // for version in &published {
            //     let tag = git::tag_for(member.name(), version, tag_format);
            //     if !repository.tags().contains(&tag) && *version >= zero_one_zero {
            //         panic!("missing tag `{}`", tag);
            //     }
            // }
            info!(log, "TODO: identify missing tags here")
        } else {
            warn!(log, "NO TAGGING = {}", member.name());
            // repository.wut();
        }

        // * Get initial supported version.
        // * Get list of published crates after that
        // * Ensure tags for each
        // * If changelog, check format

        /*
        if member.has_changelog() {
            member.unpublished(&repository);
        }
        */
    }
}
