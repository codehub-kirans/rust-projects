use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
//use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    //assert!(true);
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
    .failure()
    .stderr(predicate::str::contains("USAGE"));

    Ok(()) //or return Ok(());
    
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello Kiran").assert().success();

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.arg("Hello there").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);

    Ok(())
}