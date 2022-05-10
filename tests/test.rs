extern crate predicates;
extern crate assert_cmd;

use assert_cmd::Command;
use std::fs;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn echo_no_args() -> TestResult {
    Command::cargo_bin("echorust")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echorust")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn hello1() -> TestResult {
    run(&["Hello  there"], "tests/files/Hello.txt")
}

// --------------------------------------------------
#[test]
fn hello2() -> TestResult {
    run(&["Hello", "Mainak"], "tests/files/Hello1.txt")
}

// --------------------------------------------------
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hi", "Kasturi!"], "tests/files/Hi-k.txt")
}


