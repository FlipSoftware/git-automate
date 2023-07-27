use std::process::Output;
use std::str;

pub mod fail;

pub fn run_git_command(args: &[&str]) -> Result<Output, std::io::Error> {
    std::process::Command::new("git").args(args).output()
}

pub fn git_command(args: &[&str], error_message: &str) -> Result<String, fail::IOFail> {
    let result = run_git_command(args)?;
    let stdout = String::from_utf8(result.stdout).map_err(|e| fail::IOFail {
        message: format_args!("{error_message}\nReturned: {e}")
            .as_str()
            .unwrap(),
    })?;
    Ok(stdout)
}

pub fn git_status(args: &[&str]) -> Result<String, fail::IOFail> {
    git_command(&[&["status"], args].concat(), "git status failed")
}

pub fn git_log(args: &[&str]) -> Result<String, fail::IOFail> {
    git_command(&[&["log"], args].concat(), "git log failed")
}

pub fn git_diff(args: &[&str]) -> Result<String, fail::IOFail> {
    git_command(&[&["diff"], args].concat(), "git diff failed")
}

pub fn git_simple_commit(args: &[&str]) -> Result<String, fail::IOFail> {
    if args.is_empty() {
        return Err(fail::IOFail {
            message: "\n\nMessage can not be empty\n\n",
        });
    }
    git_command(
        &[&["commit", "--message"], args].concat(),
        "git commit failed",
    )
}

pub fn git_semantic_commit(
    args: &[&str],
    r#type: &str,
    scope: &str,
    emphasize: bool,
) -> Result<String, fail::IOFail> {
    if args.is_empty() {
        return Err(fail::IOFail {
            message: "\n\nMessage can not be empty\n\n",
        });
    }
    let subject = git_command(
        &[&["commit", "--message"], args].concat(),
        "git commit failed",
    )?;
    // Sanitize message
    let (l_offset, r_offset) = (
        &subject.find("] ").unwrap() + 2,
        &subject.find('\n').unwrap(),
    ); // last character of stdout before the subject message. +2 to skip these characters
    let subject = subject.get(l_offset..*r_offset).unwrap();

    match scope.is_empty() {
        true => {
            let mut semantic_commit = format!("`{type}`: {subject}");
            if !emphasize {
                semantic_commit = format!("{type}: {subject}");
            }
            Ok(semantic_commit)
        }
        false => {
            let mut semantic_commit = format!("`{type}` (`{scope}`): {subject}");
            if !emphasize {
                semantic_commit = format!("{type} ({scope}): {subject}");
            }
            Ok(semantic_commit)
        }
    }
}
