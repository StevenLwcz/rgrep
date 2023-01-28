use assert_cmd::Command;

const PATTERN_NOT_FOUND: i32 = 1;
const BAD_PATTERN: i32 = 2;
// const BAD_GLOB_PATTERN: i32 = 3;
// const OPEN_FILE_ERROR: i32 = 4;

#[test]
fn test1() {
    // basic it works test
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd
    .arg("qi")
    .write_stdin("pink")
    .assert()
    .success()
    .stdout("pink\n");
}

#[test]
fn test2() {
    // detects  bad regular expressiom
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd
    .arg("[pi")
    .assert()
    .failure()
    .code(BAD_PATTERN);
}

#[test]
fn test3() {
    // test 1 is returned if expression not found
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd
    .arg("pi")
    .write_stdin("cheese")
    .assert()
    .failure()
    .code(PATTERN_NOT_FOUND);
}
