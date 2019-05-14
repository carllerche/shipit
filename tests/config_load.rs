mod support;
use self::support::*;

use assertive::assert_ok;

#[test]
fn single_crate_manifest() {
    let fixture = fixture::template("basic_manifest");

    fixture.write_file("shipit.toml", "\
    packages = [\n\
        \"basic-manifest\",\n\
    ]\n\n\

    [git]\n\
    tag-format = \"v{version}\"\
    ");

    let config = assert_ok!(fixture.config());
    let project = &config.project;

    assert!(project.initial_commit.is_none());
    assert_eq!(1, project.packages.len());
}
