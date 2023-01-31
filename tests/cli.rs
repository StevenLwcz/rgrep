use assert_cmd::Command;
use std::fs;

const PATTERN_NOT_FOUND: i32 = 1;
const BAD_PATTERN: i32 = 2;
// const BAD_GLOB_PATTERN: i32 = 3;
// const OPEN_FILE_ERROR: i32 = 4;

#[test]
fn test1() {
    // basic it works test from stdin
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("pi")
        .write_stdin("alpha\nbeta\npink\ndelta\ngamma")
        .assert()
        .success()
        .stdout("pink\n");
}

#[test]
fn test2() {
    // detects bad regular expressiom and error goes to stderr
    let testfile = "tests/expected/test2.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("[pi")
        .assert()
        .failure()
        .code(BAD_PATTERN)
        .stderr(expected);
}

#[test]
fn test3() {
    // test 1 is returned if expression not found
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("pi")
        .write_stdin("cheese")
        .assert()
        .failure()
        .code(PATTERN_NOT_FOUND);
}

#[test]
fn test4() {
    // test **/*.txt glob expansion
    let testfile = "tests/expected/test4.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("red")
        .arg("**/*.txt")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test5() {
    // test single file
    let testfile = "tests/expected/test5.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("re")
        .arg("tests/files/fruits.txt")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test6() {
    // test multi file
    let testfile = "tests/expected/test4.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("red")
        .arg("tests/files/fruits.txt")
        .arg("tests/files/rainbow.txt")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test7() {
    // test --ignore
    let testfile = "tests/expected/test4.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("--ignore")
        .arg("RED")
        .arg("tests/files/fruits.txt")
        .arg("tests/files/rainbow.txt")
        .assert()
        .success()
        .stdout(expected);
}
