use assert_cmd::Command;
use std::fs;

const PATTERN_NOT_FOUND: i32 = 1;
const BAD_PATTERN: i32 = 2;
const BAD_FILE_PATTERN: i32 = 3;

/*
 * TODO sort stdout so it is not dependant on order of files in the file system
 * Get tests working on Windows
 */

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
    // test \.txt$ expression finds all .txt files
    let testfile = "tests/expected/test4.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("red")
        .arg(r"\.txt$")
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
        .arg("fruits.txt")
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
        .arg("fruits.txt")
        .arg("rainbow.txt")
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
        .arg(r"\.txt$")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test8() {
    // test (?i) in file pattern
    let testfile = "tests/expected/test8.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("--ignore")
        .arg("ORANGE")
        .arg(r"(?i)\.txt$")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test9() {
    // test invalid file pattern ?q is bad flag goes to stderr
    let testfile = "tests/expected/test9.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("--ignore")
        .arg("PURPLE")
        .arg(r"(?q)\.txt$")
        .assert()
        .failure()
        .code(BAD_FILE_PATTERN)
        .stderr(expected);
}

#[test]
fn test10() {
    // test --display
    let testfile = "tests/expected/test10.text";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("--display")
        .arg("red")
        .arg(r"\.txt$")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test11() {
    // test -v and multiple file patterns
    let testfile1 = "tests/expected/test11.text";
    let testfile2 = "tests/expected/test11.stderr.text";
    let expected1 = fs::read_to_string(testfile1).unwrap();
    let expected2 = fs::read_to_string(testfile2).unwrap();
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd.arg("-v")
        .arg(r"\bi\w+o\b")
        .arg(r"\.(rs|py)$")
        .arg(r"\.txt$")
        .assert()
        .success()
        .stdout(expected1)
        .stderr(expected2);
}
