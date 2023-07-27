use std::process::Output;

use miette::IntoDiagnostic;

use git_automate::*;

// * Helper function to prepare execution before each test
fn prepare_test(workdir: &str, file: Option<&str>) -> miette::Result<()> {
    let file = file.unwrap_or_default();
    std::env::set_current_dir("/var/tmp").into_diagnostic()?;
    miette::set_panic_hook();

    if std::fs::read_dir(workdir).is_ok() {
        std::fs::remove_dir_all(workdir).into_diagnostic()?;
    }

    let init = test_git_command(&["init", workdir])?;
    assert!(init.status.success(), "git init failed");

    if !file.is_empty() {
        let file_path = format!("{workdir}/{file}");
        let output = std::fs::write(file_path, "Hello world!");
        assert!(output.is_ok(), "writing to file failed");

        std::env::set_current_dir(format!("/var/tmp/{workdir}")).into_diagnostic()?;

        let stage_files = test_git_command(&["add", "."])?;
        assert!(stage_files.status.success(), "add to staging failed");
    }

    Ok(())
}

// * Helper function to abstract and test git commands with diagnostics
fn test_git_command(args: &[&str]) -> miette::Result<Output> {
    std::process::Command::new("git")
        .args(args)
        .output()
        .into_diagnostic()
}

#[test]
fn test_git_status_yet_to_commit() -> miette::Result<()> {
    let workdir = "test_git_status";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;
    // Command from the library to test
    let status = git_status(&["--short", "--branch"]);
    assert!(status.is_ok(), "git status failed");

    let branch = test_git_command(&["branch", "--show-current"])?;
    assert!(branch.status.success(), "git branch failed");
    let branch = std::str::from_utf8(&branch.stdout).into_diagnostic()?;

    let expected_status = format!("## No commits yet on {branch}A  {file}\n");
    assert_eq!(status.unwrap(), expected_status);
    Ok(())
}

#[test]
fn test_git_status_clean_tree() -> miette::Result<()> {
    let workdir = "test_git_status";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;

    let commit = test_git_command(&["commit", "-m", "Initial commit"])?;
    assert!(commit.status.success(), "git commit failed");
    // Command from the library to test
    let status = git_status(&["--branch"]);
    assert!(status.is_ok(), "git status failed");

    let branch = test_git_command(&["branch", "--show-current"])?;
    assert!(branch.status.success(), "git branch failed");
    let branch = std::str::from_utf8(&branch.stdout).into_diagnostic()?;

    let expected_status = format!("On branch {branch}nothing to commit, working tree clean\n");
    assert_eq!(status.unwrap(), expected_status);
    Ok(())
}

#[test]
fn test_git_log() -> miette::Result<()> {
    let workdir = "test_git_log";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;

    let commit = test_git_command(&["commit", "-m", "Initial commit"])?;
    assert!(commit.status.success(), "git commit failed");
    // Command from the library to test
    let log = git_log(&["--oneline", "--max-count=1"]);
    assert!(log.is_ok(), "git log failed");

    assert_eq!(log.unwrap().split_once(' ').unwrap().1, "Initial commit\n");
    Ok(())
}

#[test]
fn test_git_diff() -> miette::Result<()> {
    let workdir = "test_git_diff";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;

    let curr_dir = std::env::current_dir().into_diagnostic()?;
    let curr_dir = curr_dir.to_str().unwrap();
    let file_path = format!("{curr_dir}/{file}");
    let add_changes = std::fs::write(file_path, "\nNew code added");
    assert!(add_changes.is_ok(), "git commit failed");

    // Command from the library to test
    let diff = git_diff(&["--shortstat"]);
    assert!(diff.is_ok(), "git diff failed");

    assert_eq!(
        diff.unwrap(),
        " 1 file changed, 2 insertions(+), 1 deletion(-)\n"
    );
    Ok(())
}

#[test]
fn test_git_simple_commit_fail() -> miette::Result<()> {
    // Command from the library to test
    let commit = git_simple_commit(&[]);
    assert!(commit.is_err(), "git commit should fail");

    assert_eq!(
        commit.map_err(|e| e.message),
        Err("\n\nMessage can not be empty\n\n")
    );
    Ok(())
}

#[test]
fn test_git_simple_commit() -> miette::Result<()> {
    let workdir = "test_git_simple_commit";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;

    // Command from the library to test
    let commit = git_simple_commit(&["Simple commit"]);
    assert!(commit.is_ok(), "git commit failed");

    assert!(commit.unwrap().contains("Simple commit"));
    Ok(())
}

#[test]
fn test_git_semantic_commit_fail() -> miette::Result<()> {
    // Command from the library to test
    let commit = git_semantic_commit(&[], "fix", "new_file", false);
    assert!(commit.is_err(), "git commit should fail");

    assert_eq!(
        commit.map_err(|e| e.message),
        Err("\n\nMessage can not be empty\n\n")
    );
    Ok(())
}

#[test]
fn test_git_semantic_commit() -> miette::Result<()> {
    let workdir = "test_git_semantic_commit";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;

    // Command from the library to test
    let commit = git_semantic_commit(&["add fix for new_file"], "fix", "new_file", false);
    assert!(commit.is_ok(), "git commit failed");

    assert_eq!(commit.unwrap(), "fix (new_file): add fix for new_file");
    Ok(())
}

#[test]
fn test_git_semantic_commit_emphasized() -> miette::Result<()> {
    let workdir = "test_git_semantic_commit";
    let file = "new_file.txt";
    prepare_test(workdir, Some(file))?;

    // Command from the library to test
    let commit = git_semantic_commit(&["add fix for new_file"], "fix", "new_file", true);
    assert!(commit.is_ok(), "git commit failed");

    assert_eq!(commit.unwrap(), "`fix` (`new_file`): add fix for new_file");
    Ok(())
}
