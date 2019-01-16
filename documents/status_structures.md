### Data in Status 

> Workspace and Config

Workspace is contextual data passed from the application

```
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
```

Config is derived from .shipit.toml of target directory, generated with `init`

```
# Automated CHANGELOG management
[changelog]

[git]
tag-format = "name-version"

[packages.tokio]
managed-version = "0.1.14"

[packages.tokio-async-await]
managed-version = "0.1.5"

[packages.tokio-buf]
managed-version = "0.1.0"

[packages.tokio-channel]
managed-version = "0.1.0"

[packages.tokio-codec]
managed-version = "0.1.1"

[packages.tokio-current-thread]
managed-version = "0.1.4"

[packages.tokio-executor]
managed-version = "0.1.6"

[packages.tokio-fs]
managed-version = "0.1.5"

[packages.tokio-io]
managed-version = "0.1.11"

[packages.tokio-reactor]
managed-version = "0.1.8"

[packages.tokio-signal]
managed-version = "0.2.7"

[packages.tokio-tcp]
managed-version = "0.1.3"

[packages.tokio-threadpool]
managed-version = "0.1.10"

[packages.tokio-timer]
managed-version = "0.2.8"

[packages.tokio-tls]
managed-version = "0.2.1"

[packages.tokio-udp]
managed-version = "0.1.3"

[packages.tokio-uds]
managed-version = "0.2.5"
```