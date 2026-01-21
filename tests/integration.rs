use assert_cmd::Command;

#[test]
fn test_main_execution() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("photo-statistics").unwrap());
    cmd.assert().success();
}
