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

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
    .args(args)
    .assert()
    .success()
    .stdout(expected);

    Ok(())
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
    run(&["Hello there"], outfile)

}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    run(&["Hello", "there"], outfile)
}

#[test]
fn hello3() -> TestResult {
    let outfile = "tests/expected/hello1.n.txt";
    run(&["Hello  there", "-n"], outfile)
}

#[test]
fn hello4() -> TestResult {
    let outfile = "tests/expected/hello2.n.txt";
    run(&["-n", "Hello", "there"], outfile)
}