use miette::IntoDiagnostic;
use std::process::Command;
use std::str;

use git_automate::*;

#[test]
fn test_git_status() -> miette::Result<()> {
    std::env::set_current_dir("/var/tmp").into_diagnostic()?;
    miette::set_panic_hook();

    let workdir = "test_git_status";
    if std::fs::read_dir(workdir).is_ok() {
        std::fs::remove_dir_all(workdir).into_diagnostic()?;
    }

    let init = std::process::Command::new("git")
        .args(["init", workdir])
        .output()
        .into_diagnostic()?;
    assert!(init.status.success(), "git init failed");

    let file = "new_file.txt";
    let file_path = format!("{workdir}/{file}");
    let output = std::fs::write(file_path, "Hello world!");
    assert!(output.is_ok(), "writing to file failed");

    std::env::set_current_dir(format!("/var/tmp/{workdir}")).into_diagnostic()?;

    let add_file = std::process::Command::new("git")
        .args(["add", "."])
        .output()
        .into_diagnostic()?;
    assert!(add_file.status.success(), "add file failed");

    let status = git_status(&["status", "--short", "--branch"]);
    assert!(status.is_ok(), "git status failed");

    let branch = std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .into_diagnostic()?;
    assert!(branch.status.success(), "git branch faield");

    let branch = std::str::from_utf8(&branch.stdout).into_diagnostic()?;

    assert_eq!(status.unwrap().as_str(), format!("## No commits yet on {branch}A  {file}\n"));
    Ok(())
}
