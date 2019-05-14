use shipit::Workspace;
use std::path::{Path, PathBuf};

#[test]
fn basic_manifest() {
    let workspace = Workspace::load(&path("basic_manifest"));

    assert_eq!(1, workspace.members().len());

    let package = &workspace.get_member("basic-manifest").unwrap();
    assert_eq!(package.name(), "basic-manifest");
    assert_eq!(package.manifest_version().to_string(), "0.1.0");
}

fn path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{}", name))
}
