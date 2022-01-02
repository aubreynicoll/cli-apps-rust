use assert_cmd::prelude::*;
use assert_cmd::Command;
use assert_fs::fixture::FileWriteStr;
use assert_fs::NamedTempFile;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn missing_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foo").arg("not_a_file.txt");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error opening file \"not_a_file.txt\"",
    ));

    Ok(())
}

#[test]
fn print_matching_lines() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    let file = NamedTempFile::new("test_file.txt")?;
    file.write_str("test data\nmore data\nsome other bullshit\n")?;

    cmd.arg("data").arg(file.path());
    cmd.assert().success().stdout("test data\nmore data\n");

    Ok(())
}

#[test]
fn empty_pattern() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    let file = NamedTempFile::new("test_file.txt")?;
    file.write_str("test data\nmore data\nsome other bullshit\n")?;

    cmd.arg("").arg(file.path());
    cmd.assert()
        .success()
        .stdout("test data\nmore data\nsome other bullshit\n");

    Ok(())
}
