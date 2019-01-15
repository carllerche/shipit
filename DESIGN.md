# Shipit - Opinionated release management for Rust crates

Shipit is intended to aid in the release management process of Rust projects
including Tokio, Tower, and Mio. While it offers some level of configuration, it
is opinionated in the process that developers will take. Projects that stick
with the Shipit process will gain automated publishing to crates.io including
changelog generation.

## Project workflow with Shipit.

In order to work with Shipit, projects should be contained in a [Git] repository
and structured to follow [Cargo] idioms. Shipit supports both single crates and
[Cargo workspaces][workspace] containing multiple crates. When the repository
contains a workspace with multiple crates, additional requirements are described
below.

Shipit requires that the master branch is always able to be released. This means
that changes must be fully implemented in branches and merged in PRs. It also
means that breaking changes should not be applied to the master branch until the
project maintainers are ready for a breaking change release to be issued.

Shipit can be added to a project at any time. As such, it must be tolerant of
conventions used by the project before it adopted Shipit. However, it is
required that the project switches to Shipit conventions at the point of
adoption.

### Github workflow

Projects **must** be hosted on [Github]. All changes **must** be submited via
[pull requests][pr]. The [Github] repository **should** be configured to protect
the master branch, preventing commits to be pushed directly to the master
branch.

#### Labels

Shipit requires each pull request to be labeled with a categorization label. By
default, the options are:

* bug
* enhancement
* chore
* docs

This label allows Shipit to correctly generate the changelog entry.

Shipit also uses the `breaking-change` label to identify PRs that alter the
public API of a crate in a non backwards compatible way.

### Workspaces

TODO: Describe process from [tokio-rs/tokio#765](https://github.com/tokio-rs/tokio/issues/765).

## Shipit at a high level

Shipit is comprised of two components:

* A CLI application
* A Github bot (web application).

The CLI application is intended to be used by the project developer and to be
run as part of CI builds.

On each CI run, `shipit check` is run. This command verifies that the Shipit
conventions are being respected.

The Github bot responses to commands posted via comments on issues and PRs. The
Github bot also provides a status check. The status check ensures that:

* The PR has the correct labels assigned.
* The PR does not change any files managed by Shipit.

At regular intervals (configurable, default of a week). The Shipit bot prepares
a release of any unpublished changes. It does this by identifying which crates
in the workspace have unreleased changes and opens a PR that prepares the crates
for release. This includes updating the crate version and generating the
changelog entries.

A project maintainer is responsible for merging the PR. Once the PR is merged,
CI will run. At this point, using Travis deploys, the `shipit publish` command
is run. This command publishes the crates to crates.io.

Releases may be requested outside of the release interval by opening an issue
requesting a release and assigning the bot.

## Initializing a project

The `shipit init` command initializes a project for Shipit. The project must
already have a git repository initialized with Github as the origin. The output
is a `.shipit.yml` file at the root of the root of the repository with project
specific configuration values.

If initializing a crate that already has published releases to crates.io, then
.`shipit.yml` will include information indicating at what point in the history
Shipit was added. This is the first unpublished crate version and an optional
git sha. This information helps Shipit integrate with existing projects.

## Pull requests

### Opening a new pull request

When a new pull request is opened, the Shipit bot is notified via a Github
status check notification. The Github bot verifies the PR has the appropriate
labels. If the labels are not correctly set, the pull request cannot be merged
as the status check will fail.

If the pull request includes the `breaking-change` label, then the status check
ensures that the crate version has been incremented as part of the PR to reflect
the breaking change.

### Merging a pull request

When merging a pull request, the merger should ensure that the title of the pull
request is appropriate for a changelog entry. When publishing, Shipit will use
this title for the entry.

### Coordinating many breaking changes

Most projects batch breaking changes into a single breaking release. To do this,
breaking changes should be staged on a branch. Pull requests **must** be used to
apply changes to this branch in the same way as described above. When the
project is ready to publish the breaking changes, a pull request is opened to
merge the long lived branch into the master branch.

**When merging the pull request, a merge commit should be used instead of
rebasing**

## Publishing changes

Describes the automated process of releasing crates

### Opening a release pull request

At a regular interval, the Shipit bot will open pull requests to publish crates.
This pull request will make the following changes:

* Increment the crate version (if needed)
* Update the documentation URL if docs.rs is used.
* Generate changelog entries.

In the git commit, Shipit leaves the following message:

```
Release [crate-name] vX.X.X

Shipit-release: [crate-name]-X.X.X
```

If multiple crates in a workspace have unpublished changes, a single commit is
created for all of them. In this case, the message will be:

```
Release [crate-a], [crate-b], [crate-c]

Shipit-release: [crate-a]-X.X.X
Shipit-release: [crate-b]-X.X.X
Shipit-release: [crate-c]-X.X.X
```

The `Shipit-release` tag allows Shipit to identify commits that it has authored
for the purpose of releasing crates.

When the project maintainers are ready to publish, the pull request is merged.

### Publish process

TODO: what happens on CI to publish the crates.

#### Generating changelog entries

TODO: How are changelog entries generated.


[Git]: https://git-scm.com/
[Cargo]: https://doc.rust-lang.org/cargo/guide/
[workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[Github]: https://github.com/
[pr]: https://help.github.com/articles/about-pull-requests/
