use assert_cmd::Command;

#[test]
fn test1() {
    // basic it works test
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd
    .arg("pi")
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
    .code(1);
}
