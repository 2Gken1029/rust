use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// 引数なしでプログラムを実行して、使い方が標準エラーに出力されることを検証
#[test]
fn dies_no_args() -> TestResult {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.assert()
    .failure()
    .stderr(predicates::str::contains("USAGE"));
  Ok(())
}

// 引数に「hello」と与えて実行し、正常に終了することを検証する
#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.arg("hello").assert().success();
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
fn hello1() ->TestResult {
  run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() ->TestResult {
  run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() ->TestResult {
  run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() ->TestResult {
  run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}