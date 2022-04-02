use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn it_works() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rs_cli")?;
    // cmd.arg("--help");
    cmd.assert().success();
    cmd.assert()
        .stdout(predicate::str::contains("The answer is: 42"));
    cmd.assert()
        .stdout(predicate::str::contains("Hello, world!"));

    Ok(())
}
