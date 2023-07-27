use std::{process::{Command, Output}};
use std::str;

pub mod fail;

pub fn git_command(args: &[&str]) -> Result<Output, std::io::Error> {
    std::process::Command::new("git").args(args).output()
}

pub fn git_status(args: &[&str]) -> Result<String, fail::IOFail> {
    let status = git_command(args);
    let str = String::from_utf8(status?.stdout);
    str.map_err(|_| fail::IOFail { message: "git status failed" })
}