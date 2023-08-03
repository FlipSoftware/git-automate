// Write integration tests for all commands for every possible case exhaustively
use std::process::Output;
use fail::Fail;

mod fail;

pub fn run_git_command(args: &[&str]) -> Result<Output, std::io::Error> {
    std::process::Command::new("git").args(args).output()
}

pub fn git_command(args: &[&str]) -> Result<String, Fail> {
    let result = run_git_command(args)?;
    let stdout = String::from_utf8(result.stdout).map_err(Fail::from)?;
    Ok(stdout)
}

pub fn git_status(args: &[&str]) -> Result<String, Fail> {
    git_command(&[&["status"], args].concat())
}

pub fn git_log(args: &[&str]) -> Result<String, Fail> {
    git_command(&[&["log"], args].concat())
}

pub fn git_diff(args: &[&str]) -> Result<String, Fail> {
    git_command(&[&["diff"], args].concat())
}

pub fn git_staging_area(args: &[&str]) -> Result<String, Fail> {
    git_command(args)
}

pub fn git_stash(args: &[&str]) -> Result<String, Fail> {
    git_command(&[&["stash"], args].concat())
}

pub fn git_checkout(args: &[&str]) -> Result<String, Fail> {
    git_command(&[&["checkout"], args].concat())
}

pub fn git_branch(args: &[&str]) -> Result<String, Fail> {
    git_command(&[&["branch"], args].concat())
}

pub fn git_simple_commit(args: &[&str]) -> Result<String, Fail> {
    if args.is_empty() {
        return Err(Fail::Other("Commit message cannot be empty"));
    }
    git_command(&[&["commit", "--message"], args].concat())
}

pub fn git_semantic_commit(
    r#type: &str,
    scope: &str,
    md_marker: bool,
    args: &[&str],
) -> Result<String, Fail> {
    if args.is_empty() {
        return Err(Fail::Other("Commit message cannot be empty"));
    }
    let subject = git_command(&[&["commit", "--message"], args].concat())?;

    // Extract the subject message
    let (l_offset, r_offset) = (
        subject.find("] ").ok_or(Fail::Other("Invalid subject format"))? + 2,
        subject.find('\n').unwrap_or(subject.len()),
    );
    let subject = &subject[l_offset..r_offset];

    let commit_message = match scope.is_empty() {
        true => format!("`{}`: {}", r#type, subject),
        false => format!("`{}` (`{}`): {}", r#type, scope, subject),
    };

    if md_marker {
        Ok(commit_message)
    } else {
        Ok(commit_message.replace('`', ""))
    }
}
