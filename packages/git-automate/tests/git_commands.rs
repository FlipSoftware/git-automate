use miette::IntoDiagnostic;
use std::process::Command;
use std::str;

// Helper function to initialize the test repository and commit files
fn setup_test_repo(workdir: &str) -> miette::Result<()> {
    miette::set_panic_hook();

    let output = Command::new("git")
        .args(["init", workdir])
        .output()
        .into_diagnostic()?;
    assert!(output.status.success());

    let curr_dir = std::env::current_dir().into_diagnostic()?;
    let curr_dir = curr_dir.to_str().unwrap();

    // Set the full path to avoid errors
    let file_path = format!("{curr_dir}/{workdir}/new_file.txt");
    std::fs::write(&file_path, "New code").into_diagnostic()?;
    let output = Command::new("git")
        .args(["add", &file_path])
        .current_dir(workdir)
        .output()
        .into_diagnostic()?;
    assert!(output.status.success());

    let output = Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(workdir)
        .output()
        .into_diagnostic()?;
    assert!(output.status.success());

    Ok(())
}

// Helper function to execute Git commands and capture the output with miette
fn run_git_command(args: &[&str], workdir: &str) -> miette::Result<std::process::Output> {
    miette::set_panic_hook();
    Command::new("git")
        .args(args)
        .current_dir(workdir)
        .output()
        .into_diagnostic()
}

#[test]
fn test_git_status_nothing_to_do() -> miette::Result<()> {
    let workdir = "test_init";
    std::process::Command::new("git")
        .args(["init", workdir])
        .output()
        .into_diagnostic()?;

    let output = run_git_command(&["status", "--short", "--branch"], workdir)?;
    assert!(output.status.success());
    // Clean up
    std::fs::remove_dir_all(workdir).into_diagnostic()?;

    let expected_status = "## No commits yet on master";
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected_status
    );

    Ok(())
}

#[test]
fn test_git_status_with_changes() -> miette::Result<()> {
    let workdir = "test_status";
    if std::fs::read_dir(workdir).is_ok() {
        std::fs::remove_dir_all(workdir).into_diagnostic()?;
    }
    setup_test_repo(workdir)?;

    // Make some changes to files
    let file_path = format!("{workdir}/new_file.txt");
    std::fs::write(&file_path, format!("Updated code: {file_path}")).into_diagnostic()?;

    let output = run_git_command(&["status", "--short", "--branch"], workdir)?;
    assert!(output.status.success());

    let branch = std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .into_diagnostic()?;
    let branch = String::from_utf8(branch.stdout).into_diagnostic()?;
    let expected_status = format!("## {branch} M new_file.txt");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        expected_status
    );

    // Clean up
    std::fs::remove_dir_all(workdir).into_diagnostic()?;
    Ok(())
}
