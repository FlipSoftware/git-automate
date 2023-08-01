use std::fmt;
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum Fail {
    GitStatusFail(&'static str),
    GitLogFail(&'static str),
    GitDiffFail(&'static str),
    GitStagingFail(&'static str),
    GitStashFail(&'static str),
    GitCommitFail(&'static str),
    GitSemCommitFail(&'static str),
    GitBranchFail(&'static str),
    Other(&'static str),
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Fail::*;
        match self {
            GitStatusFail(extra_msg) => write!(f, "Git status failed: {}", extra_msg),
            GitLogFail(extra_msg) => write!(f, "Git log failed: {}", extra_msg),
            GitDiffFail(extra_msg) => write!(f, "Git diff failed: {}", extra_msg),
            GitStagingFail(extra_msg) => write!(f, "Git staging failed: {}", extra_msg),
            GitStashFail(extra_msg) => write!(f, "Git stash failed: {}", extra_msg),
            GitCommitFail(extra_msg) => write!(f, "Git commit failed: {}", extra_msg),
            GitSemCommitFail(extra_msg) => write!(f, "Git semantic commit failed: {}", extra_msg),
            GitBranchFail(extra_msg) => write!(f, "Git branch failed: {}", extra_msg),
            Other(extra_msg) => write!(f, "Error occurred: {}", extra_msg),
        }
    }
}

impl std::error::Error for Fail {}

impl miette::Diagnostic for Fail {
    fn code<'a>(&'a self) -> Option<Box<dyn 'a + fmt::Display>> {
        Some(Box::new(self))
    }
}

impl From<io::Error> for Fail {
    fn from(error: io::Error) -> Self {
        let message = format_args!("IO error: {error}").as_str().unwrap_or_default();
        Fail::Other(message)
    }
}

impl From<Utf8Error> for Fail {
    fn from(error: Utf8Error) -> Self {
        let message = format_args!("UTF-8 decoding error: {error}").as_str().unwrap_or_default();
        Fail::Other(message)
    }
}

impl From<FromUtf8Error> for Fail {
    fn from(error: FromUtf8Error) -> Self {
        let message = format_args!("From UTF-8 error: {error}").as_str().unwrap_or_default();
        Fail::Other(message)
    }
}

impl FromStr for Fail {
    type Err = ();

    fn from_str(error: &str) -> Result<Self, Self::Err> {
        let message = format_args!("FromStr error: {error}").as_str().unwrap_or_default();
        Ok(Fail::Other(message))
    }
}