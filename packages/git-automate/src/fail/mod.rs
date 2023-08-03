use std::fmt;
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum Fail {
    StatusError(&'static str),
    LogError(&'static str),
    DiffError(&'static str),
    StagingError(&'static str),
    StashError(&'static str),
    CommitError(&'static str),
    SemanticCommitError(&'static str),
    BranchError(&'static str),
    Other(&'static str),
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Fail::*;
        match self {
            StatusError(extra_msg) => write!(f, "Git status failed: {}", extra_msg),
            LogError(extra_msg) => write!(f, "Git log failed: {}", extra_msg),
            DiffError(extra_msg) => write!(f, "Git diff failed: {}", extra_msg),
            StagingError(extra_msg) => write!(f, "Git staging failed: {}", extra_msg),
            StashError(extra_msg) => write!(f, "Git stash failed: {}", extra_msg),
            CommitError(extra_msg) => write!(f, "Git commit failed: {}", extra_msg),
            SemanticCommitError(extra_msg) => write!(f, "Git semantic commit failed: {}", extra_msg),
            BranchError(extra_msg) => write!(f, "Git branch failed: {}", extra_msg),
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