use std::process::{Command, Stdio};

fn main() {
    let mut cargo_test_command = Command::new("cargo");
    cargo_test_command.current_dir("/home/ab/Documents/GitLab/always-recompile-issue");
    cargo_test_command.arg("test");
    cargo_test_command.arg("--workspace");
    cargo_test_command.arg("--tests");
    cargo_test_command.env("RUSTFLAGS", "-C instrument-coverage");
    cargo_test_command.stdout(Stdio::inherit());
    cargo_test_command.stderr(Stdio::inherit());
    cargo_test_command.spawn().unwrap().wait().unwrap();
    dbg!(cargo_test_command.get_current_dir());
}

#[test]
#[ignore]
fn test() {
    let mut cargo_test_command = Command::new("cargo");
    cargo_test_command.current_dir("/home/ab/Documents/GitLab/always-recompile-issue");
    cargo_test_command.arg("test");
    cargo_test_command.arg("--workspace");
    cargo_test_command.arg("--tests");
    cargo_test_command.env("RUSTFLAGS", "-C instrument-coverage");
    cargo_test_command.stdout(Stdio::inherit());
    cargo_test_command.stderr(Stdio::inherit());
    cargo_test_command.spawn().unwrap().wait().unwrap();
    dbg!(cargo_test_command.get_current_dir());
}
