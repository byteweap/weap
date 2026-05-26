use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    Command::cargo_bin("weap")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("一个简单实用的命令行小工具"));
}

#[test]
fn test_cli_version() {
    Command::cargo_bin("weap")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("weap"));
}

#[test]
fn test_cli_uuid() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["uuid"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\n$").unwrap());
}

#[test]
fn test_cli_uuid_no_hyphens() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["uuid", "-x"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{32}\n$").unwrap());
}

#[test]
fn test_cli_pw() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["pw"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^.{16}\n$").unwrap());
}

#[test]
fn test_cli_pw_custom_length() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["pw", "-l", "32"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^.{32}\n$").unwrap());
}

#[test]
fn test_cli_pw_with_special() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["pw", "-s"])
        .assert()
        .success();
}

#[test]
fn test_cli_sys() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["sys"])
        .assert()
        .success()
        .stdout(predicate::str::contains("操作系统:"))
        .stdout(predicate::str::contains("CPU:"));
}

#[test]
fn test_cli_time() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["time"])
        .assert()
        .success()
        .stdout(predicate::str::contains("当前本地时间:"))
        .stdout(predicate::str::contains("当前UTC时间:"));
}

#[test]
fn test_cli_time_with_unix() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["time", "-u"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Unix时间戳:"));
}

#[test]
fn test_cli_time_with_timezone() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["time", "-t", "Asia/Shanghai"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Asia/Shanghai时间:"));
}

#[test]
fn test_cli_time_with_invalid_timezone() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["time", "-t", "Invalid/Zone"])
        .assert()
        .success()
        .stdout(predicate::str::contains("错误: 无效的时区"));
}

#[test]
fn test_cli_time_custom_format() {
    Command::cargo_bin("weap")
        .unwrap()
        .args(&["time", "-f", "%H:%M"])
        .assert()
        .success();
}

#[test]
fn test_cli_no_args() {
    Command::cargo_bin("weap")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn test_cli_unknown_command() {
    Command::cargo_bin("weap")
        .unwrap()
        .arg("unknown")
        .assert()
        .failure();
}
