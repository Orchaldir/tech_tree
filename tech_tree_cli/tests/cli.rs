use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn unknown_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tech_tree_cli")?;

    cmd.arg("unknown/file.yaml");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to read \"unknown/file.yaml\"",
    ));

    Ok(())
}

#[test]
fn invalid_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Invalid content!")?;

    let mut cmd = Command::cargo_bin("tech_tree_cli")?;

    cmd.arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse"));

    Ok(())
}

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "---\n
technologies:\n
  - name: Technology 0\n
    predecessors: []"
    )?;

    let mut cmd = Command::cargo_bin("tech_tree_cli")?;

    cmd.arg(file.path());
    cmd.assert().success();

    Ok(())
}
