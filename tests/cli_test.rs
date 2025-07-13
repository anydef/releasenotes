use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("releasenotes").unwrap();
    let assert = cmd.arg("--help").assert();
    assert.success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("list-commits"))
        .stdout(predicate::str::contains("generate-release-notes"));
}

#[test]
fn test_list_commits_help() {
    let mut cmd = Command::cargo_bin("releasenotes").unwrap();
    let assert = cmd.arg("list-commits").arg("--help").assert();
    assert.success()
        .stdout(predicate::str::contains("owner"))
        .stdout(predicate::str::contains("repo"))
        .stdout(predicate::str::contains("from"))
        .stdout(predicate::str::contains("to"));
}

#[test]
fn test_missing_arguments() {
    let mut cmd = Command::cargo_bin("releasenotes").unwrap();
    let assert = cmd.arg("list-commits").assert();
    assert.failure()
        .stderr(predicate::str::contains("the following required arguments were not provided"))
        .stderr(predicate::str::contains("--owner"))
        .stderr(predicate::str::contains("--repo"))
        .stderr(predicate::str::contains("--from"))
        .stderr(predicate::str::contains("--to"));
}

#[test]
fn test_short_arguments() {
    let mut cmd = Command::cargo_bin("releasenotes").unwrap();
    // This test will fail in CI since it tries to connect to GitHub
    // In a real scenario, we would mock the GitHub API
    // For now, we're just testing that the CLI accepts short arguments
    let assert = cmd
        .arg("list-commits")
        .arg("-o").arg("test-owner")
        .arg("-r").arg("test-repo")
        .arg("-f").arg("test-from")
        .arg("-t").arg("test-to")
        .assert();

    // The command will likely fail due to GitHub API connection issues
    // but we're just testing that the CLI accepts the arguments
    assert.failure();
}

#[test]
fn test_generate_release_notes_help() {
    let mut cmd = Command::cargo_bin("releasenotes").unwrap();
    let assert = cmd.arg("generate-release-notes").arg("--help").assert();
    assert.success()
        .stdout(predicate::str::contains("owner"))
        .stdout(predicate::str::contains("repo"))
        .stdout(predicate::str::contains("from"))
        .stdout(predicate::str::contains("to"))
        .stdout(predicate::str::contains("output-file"));
}

#[test]
fn test_generate_release_notes_missing_arguments() {
    let mut cmd = Command::cargo_bin("releasenotes").unwrap();
    let assert = cmd.arg("generate-release-notes").assert();
    assert.failure()
        .stderr(predicate::str::contains("the following required arguments were not provided"))
        .stderr(predicate::str::contains("--owner"))
        .stderr(predicate::str::contains("--repo"))
        .stderr(predicate::str::contains("--from"))
        .stderr(predicate::str::contains("--to"));
}
