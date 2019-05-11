use shipit::action;

mod support;
use self::support::*;

#[test]
fn basic_crate() {
    let fixture = fixture::template("basic_manifest");
    let workspace = fixture.workspace();

    action::init::run(&workspace, None);

    let config = read_file(fixture.path().join(".shipit.toml"));

    assert_eq!(config, "\
    # Automated CHANGELOG management\n\
    [changelog]\n\n\

    [git]\n\
    tag-format = \"version\"\n\n\

    [packages.basic-manifest]\n\
    managed-version = \"0.1.0\"\n\
    ");
}
