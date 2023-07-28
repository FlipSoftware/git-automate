use miette::IntoDiagnostic;
use std::process::Output;

use git_automate::*;

// * Helper function to prepare execution before each test
fn prepare_test(workdir: &str, file: Option<&str>) -> miette::Result<()> {
    let file = file.unwrap_or_default();
    std::env::set_current_dir("/tmp").into_diagnostic()?;
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

        std::env::set_current_dir(format!("/tmp/{workdir}")).into_diagnostic()?;

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
    let (workdir, file) = ("test_git_status", "new_file.txt");
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
    let (workdir, file) = ("test_git_status", "new_file.txt");
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
    let (workdir, file) = ("test_git_log", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    let commit = test_git_command(&["commit", "-m", "Initial commit"])?;
    assert!(commit.status.success(), "git commit failed");
    // Command from the library to test
    let log = git_log(&["--oneline", "--max-count=1"]);
    assert!(log.is_ok(), "git log failed");

    let expected_log = "Initial commit\n";
    assert_eq!(log.unwrap().split_once(' ').unwrap().1, expected_log);
    Ok(())
}

#[test]
fn test_git_diff() -> miette::Result<()> {
    let (workdir, file) = ("test_git_diff", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    let curr_dir = std::env::current_dir().into_diagnostic()?;
    let curr_dir = curr_dir.to_str().unwrap();
    let file_path = format!("{curr_dir}/{file}");
    let add_changes = std::fs::write(file_path, "\nNew code added");
    assert!(add_changes.is_ok(), "git commit failed");

    // Command from the library to test
    let diff = git_diff(&["--shortstat"]);
    assert!(diff.is_ok(), "git diff failed");

    let expected_diff = " 1 file changed, 2 insertions(+), 1 deletion(-)\n";
    assert_eq!(diff.unwrap(), expected_diff);
    Ok(())
}

#[test]
fn test_git_stage_all() -> miette::Result<()> {
    let workdir = "test_git_add_all";
    prepare_test(workdir, None)?;

    let curr_dir = std::env::current_dir().unwrap(); // -> /tpm
    let curr_dir = curr_dir.to_str().unwrap();
    let change_dir = std::env::set_current_dir(format!("{curr_dir}/{workdir}"));
    assert!(change_dir.is_ok(), "change directory failed");

    let file_a = "new_file.txt";
    let write = std::fs::write(file_a, "Hello world!");
    assert!(write.is_ok(), "write to file failed");
    let file_b = "lib.rs";
    let write = std::fs::write(file_b, "fn hello() -> String { 'Hello'.to_string() }");
    assert!(write.is_ok(), "write to file failed");
    // Command from library to test
    let stage_all = git_staging_area(&["add", "--all", "--verbose"]);
    assert!(stage_all.is_ok(), "git add failed");

    let expected_stage_all = format!("add '{file_b}'\nadd '{file_a}'\n");
    assert_eq!(stage_all.unwrap(), expected_stage_all);
    Ok(())
}

#[test]
fn test_git_stage_single_file() -> miette::Result<()> {
    let workdir = "test_git_add_single";
    prepare_test(workdir, None)?;

    let curr_dir = std::env::current_dir().unwrap(); // -> /tpm
    let curr_dir = curr_dir.to_str().unwrap();
    let change_dir = std::env::set_current_dir(format!("{curr_dir}/{workdir}"));
    assert!(change_dir.is_ok(), "change directory failed");

    let file = "new_file.txt";
    let write = std::fs::write(file, "Hello world!");
    assert!(write.is_ok(), "write to file failed");

    let stage_single = git_staging_area(&["add", file, "--verbose"]);
    assert!(stage_single.is_ok(), "git add failed");

    let expected_stage_single = format!("add '{file}'\n");
    assert_eq!(stage_single.unwrap(), expected_stage_single);
    Ok(())
}

#[test]
fn test_git_stage_restore_all() -> miette::Result<()> {
    let workdir = "test_git_restore_all";
    prepare_test(workdir, None)?;

    let curr_dir = std::env::current_dir().unwrap(); // -> /tpm
    let curr_dir = curr_dir.to_str().unwrap();
    let change_dir = std::env::set_current_dir(format!("{curr_dir}/{workdir}"));
    assert!(change_dir.is_ok(), "change directory failed");

    let file_a = "new_file.txt";
    let write = std::fs::write(file_a, "Hello world!");
    assert!(write.is_ok(), "write to file failed");
    let file_b = "lib.rs";
    let write = std::fs::write(file_b, "fn hello() -> String { 'Hello'.to_string() }");
    assert!(write.is_ok(), "write to file failed");

    let stage_all = git_staging_area(&["add", ".", "--verbose"]);
    assert!(stage_all.is_ok(), "git add failed");
    // Command from library to test
    let restore_all = git_staging_area(&["restore", "--staged", "."]);
    assert!(restore_all.is_ok(), "git restore failed");

    let restore_all_status = git_status(&["--short"]);
    assert!(restore_all_status.is_ok(), "git status failed");

    let expected_status = format!("A  {file_b}\nA  {file_a}\n");
    assert_eq!(restore_all_status.unwrap(), expected_status);
    Ok(())
}

#[test]
fn test_git_stage_restore_single_file() -> miette::Result<()> {
    let workdir = "test_git_restore_single";
    prepare_test(workdir, None)?;

    let curr_dir = std::env::current_dir().unwrap(); // -> /tpm
    let curr_dir = curr_dir.to_str().unwrap();
    let change_dir = std::env::set_current_dir(format!("{curr_dir}/{workdir}"));
    assert!(change_dir.is_ok(), "change directory failed");

    let file = "new_file.txt";
    let write = std::fs::write(file, "Hello world!");
    assert!(write.is_ok(), "write to file failed");

    let stage_single = git_staging_area(&["add", file, "--verbose"]);
    assert!(stage_single.is_ok(), "git add failed");
    // Command from library to test
    let restore_single = git_staging_area(&["restore", "--staged", file]);
    assert!(restore_single.is_ok(), "git restore failed");

    let restore_single_status = git_status(&["--short"]);
    assert!(restore_single_status.is_ok(), "git status failed");

    let expected_status = format!("A  {file}\n");
    assert_eq!(restore_single_status.unwrap(), expected_status);
    Ok(())
}

#[test]
fn test_git_stash_show() -> miette::Result<()> {
    let (workdir, file) = ("test_git_stash_show", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    let commit = git_simple_commit(&["Initial commit"]);
    assert!(commit.is_ok(), "git commit failed");

    // Change file contents
    let write = std::fs::write(file, "fn hello() -> String { 'Hello'.to_string() }");
    assert!(write.is_ok(), "write to file failed");

    // Command from library to test
    let _ = git_stash(&[]);
    let stash = git_stash(&["show", "--include-untracked"]);
    assert!(stash.is_ok(), "git stash failed");

    let expected_stash =
        format!(" {file} | 2 +-\n 1 file changed, 1 insertion(+), 1 deletion(-)\n");
    assert_eq!(stash.unwrap(), expected_stash);
    Ok(())
}

#[test]
fn test_git_stash_pop() -> miette::Result<()> {
    let (workdir, file) = ("test_git_stash_show", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    let commit = git_simple_commit(&["Initial commit"]);
    assert!(commit.is_ok(), "git commit failed");

    // Change file contents
    let write = std::fs::write(file, "fn hello() -> String { 'Hello'.to_string() }");
    assert!(write.is_ok(), "write to file failed");

    // Command from library to test
    let _ = git_stash(&[]);
    let stash = git_stash(&["pop"]);
    assert!(stash.is_ok(), "git stash failed");

    assert!(stash.unwrap().contains("Dropped refs/stash"));
    Ok(())
}

#[test]
fn test_git_stash_drop() -> miette::Result<()> {
    let (workdir, file) = ("test_git_stash_apply", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    let commit = git_simple_commit(&["Initial commit"]);
    assert!(commit.is_ok(), "git commit failed");

    // Change file contents
    let write = std::fs::write(file, "fn hello() -> String { 'Hello'.to_string() }");
    assert!(write.is_ok(), "write to file failed");

    // Command from library to test
    let _ = git_stash(&[]);
    let stash = git_stash(&["drop"]);
    assert!(stash.is_ok(), "git stash failed");

    assert!(stash.unwrap().contains("Dropped refs/stash"));
    Ok(())
}

#[test]
fn test_git_simple_commit_fail() -> miette::Result<()> {
    // Command from the library to test
    let commit = git_simple_commit(&[]);
    assert!(commit.is_err(), "git commit should fail");

    let expected_commit_err = Err("\n\nMessage can not be empty\n\n");
    assert_eq!(commit.map_err(|e| e.message), expected_commit_err);
    Ok(())
}

#[test]
fn test_git_simple_commit() -> miette::Result<()> {
    let (workdir, file) = ("test_git_simple_commit", "new_file.txt");
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
    let (r#type, scope, md_marker) = ("fix", "new_file", false);
    let commit = git_semantic_commit(r#type, scope, md_marker, &[]);
    assert!(commit.is_err(), "git commit should fail");

    let expected_commit_err = Err("\n\nMessage can not be empty\n\n");
    assert_eq!(commit.map_err(|e| e.message), expected_commit_err);
    Ok(())
}

#[test]
fn test_git_semantic_commit() -> miette::Result<()> {
    let (workdir, file) = ("test_git_semantic_commit", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    // Command from the library to test
    let (r#type, scope, md_marker, subject) = ("fix", "new_file", false, "add fix for new_file");
    let commit = git_semantic_commit(r#type, scope, md_marker, &[subject]);
    assert!(commit.is_ok(), "git commit failed");

    let expected_commit = format!("{type} ({scope}): {subject}");
    assert_eq!(commit.unwrap(), expected_commit);
    Ok(())
}

#[test]
fn test_git_semantic_commit_no_scope() -> miette::Result<()> {
    let (workdir, file) = ("test_git_semantic_commit", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    // Command from the library to test
    let (r#type, scope, md_marker, subject) = ("fix", "", false, "add fix for new_file");
    let commit = git_semantic_commit(r#type, scope, md_marker, &[subject]);
    assert!(commit.is_ok(), "git commit failed");

    let expected_commit = format!("{type}: {subject}");
    assert_eq!(commit.unwrap(), expected_commit);
    Ok(())
}

#[test]
fn test_git_semantic_commit_md_marker_no_scope() -> miette::Result<()> {
    let (workdir, file) = ("test_git_semantic_commit", "new_file.txt");
    prepare_test(workdir, Some(file))?;

    // Command from the library to test
    let (r#type, scope, md_marker, subject) = ("fix", "", true, "add fix for new_file");
    let commit = git_semantic_commit(r#type, scope, md_marker, &[subject]);
    assert!(commit.is_ok(), "git commit failed");

    let expected_commit = format!("`{type}`: {subject}");
    assert_eq!(commit.unwrap(), expected_commit);
    Ok(())
}
