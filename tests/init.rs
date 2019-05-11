use shipit::action;

mod support;
use self::support::*;

#[test]
fn basic_crate() {
    let fixture = fixture::template("basic_manifest");
    let workspace = fixture.workspace();

    action::init::run(&workspace, None);

    let config = read_file(fixture.path().join(".shipit.toml"));

    assert_eq!(config, "");
}
