use assert_cmd::Command;

#[test]
fn runs() {
    // detects  bad regular expressiom
    let mut cmd = Command::cargo_bin("grepr").unwrap();
    cmd
    .arg("pi")
    .write_stdin("pink")
    .assert()
    .success()
    .stdout("pink\n");
}
