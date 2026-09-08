//! The status line must never panic, not even when whoever reads its stdout
//! goes away first. Claude Code does not do that, but a shell pipeline does.

use std::io::Write;
use std::process::{Command, Stdio};

fn run_with_closed_stdout(args: &[&str], stdin_data: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_klaude-status"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn klaude-status");

    // Arrange: close the reading end before the child writes anything.
    drop(child.stdout.take());

    // Act: feed stdin and let the child finish.
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(stdin_data.as_bytes());
    }
    child.wait_with_output().expect("wait for klaude-status")
}

#[test]
fn exits_cleanly_when_stdout_reader_closes_first() {
    let output = run_with_closed_stdout(&[], r#"{"cwd":"/tmp","model":{"display_name":"Fable"}}"#);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "panicked on broken pipe: {stderr}"
    );
    assert!(output.status.success(), "exit status: {}", output.status);
}

#[test]
fn demo_exits_cleanly_when_stdout_reader_closes_first() {
    let output = run_with_closed_stdout(&["--demo"], "");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "panicked on broken pipe: {stderr}"
    );
    assert!(output.status.success(), "exit status: {}", output.status);
}
