use fail::Fail;
use std::{io, process};

pub mod fail;

#[derive(Debug, Clone, Copy, Default)]
pub struct GitCommand;

impl GitCommand {
    pub fn new() -> Self {
        Self
    }
}

pub trait GitOperations {
    /// #### Run a Git command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - A string containing the arguments for the Git command.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.run_git_command("remote --verbose")?;
    /// println!("Git Remote Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn run_git_command(&self, args: &str) -> Result<process::Output, io::Error> {
        let splited_args = args.trim().split(' ').map(|ch| ch.replace("%s#", " "));
        process::Command::new("git").args(splited_args).output()
    }

    /// #### Run a Git `status` command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git status command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_status("")?;
    /// println!("Git Status Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_status(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Run a Git `log` command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git log command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_log("--oneline")?;
    /// println!("Git Log Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_log(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Run a Git `diff` command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git diff command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_diff("")?;
    /// println!("Git Diff Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_diff(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Run a Git `staging` area command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git staging area command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_staging_area("status --branch --untracked-files")?;
    /// println!("Git Staging Area Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_staging_area(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Run a Git `stash` command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git stash command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_stash("save 'WIP: Work in progress' --dry-run")?;
    /// println!("Git Stash Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_stash(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Run a Git `checkout` command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git checkout command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_checkout("some-feature-branch")?;
    /// println!("Git Checkout Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_checkout(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Run a Git `branch` command and return the `process::Output`.
    /// <hr>
    ///
    /// * `args` - Additional arguments for the Git branch command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_branch("--list")?;
    /// println!("Git Branch Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_branch(&self, args: &str) -> Result<process::Output, Fail>;

    /// #### Perform a simple Git `commit`.
    /// <hr>
    ///
    /// * `message` - The commit message.
    /// * `args` - Additional arguments for the Git commit command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_simple_commit("Initial commit", "--dry-run")?;
    /// println!("Git Commit Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_simple_commit(&self, message: &str, args: &str) -> Result<process::Output, Fail>;

    /// #### Perform a semantic Git `commit` (based on `SemVer`) with optional Markdown backquotes mark (default `true`).
    /// <hr>
    ///
    /// * `r#type` - The type of the semantic commit.
    /// * `scope` - The scope of the semantic commit.
    /// * `md_marker` - Indicates if the commit message should be marked up with Markdown backquotes.
    /// * `args` - Additional arguments for the Git commit command, separated by spaces.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.git_semantic_commit("feat", "ui", true, "Initial commit", "--dry-run")?;
    /// println!("Git Commit Output:\n{}", output.stdout_to_string()?);
    /// Ok::<(), Fail>(())
    /// ```
    fn git_semantic_commit(
        &self,
        r#type: &str,
        scope: &str,
        md_marker: bool,
        message: &str,
        args: &str,
    ) -> Result<process::Output, Fail>;
}

pub trait GitStdoutExt {
    /// #### Convert the `stdout` of a `process::Output` to a `String`.
    /// <hr>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fail::Fail;
    /// use git_automate::*;
    ///
    /// let git_cmd = GitCommand::new();
    /// let output = git_cmd.run_git_command("status")?;
    /// let stdout = output.stdout_to_string()?;
    /// println!("Git Status Output:\n{}", stdout);
    /// Ok::<(), Fail>(())
    /// ```
    fn stdout_to_string(&self) -> Result<String, Fail>;
}

impl GitStdoutExt for process::Output {
    fn stdout_to_string(&self) -> Result<String, Fail> {
        String::from_utf8(self.stdout.clone()).map_err(Fail::from)
    }
}

impl GitOperations for GitCommand {
    fn git_status(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(&format!("status {args}"))
            .map_err(|e| e.into())
    }
    fn git_log(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(&format!("log {args}"))
            .map_err(|e| e.into())
    }
    fn git_diff(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(&format!("diff {args}"))
            .map_err(|e| e.into())
    }
    fn git_staging_area(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(args).map_err(|e| e.into())
    }
    fn git_stash(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(&format!("stash {args}"))
            .map_err(|e| e.into())
    }
    fn git_checkout(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(&format!("checkout {args}"))
            .map_err(|e| e.into())
    }
    fn git_branch(&self, args: &str) -> Result<process::Output, Fail> {
        self.run_git_command(&format!("branch {args}"))
            .map_err(|e| e.into())
    }
    fn git_simple_commit(&self, message: &str, args: &str) -> Result<process::Output, Fail> {
        if message.is_empty() {
            return Err(Fail::Other("Commit message cannot be empty"));
        }
        let sanitized_message = message.replace(' ', "%s#");
        self.run_git_command(&format!("commit --message {sanitized_message} {args}"))
            .map_err(|e| e.into())
    }
    fn git_semantic_commit(
        &self,
        r#type: &str,
        scope: &str,
        md_marker: bool,
        message: &str,
        args: &str,
    ) -> Result<process::Output, Fail> {
        if args.is_empty() {
            return Err(Fail::Other("Commit message cannot be empty"));
        }
        let sanitized_message = message.replace(' ', "%s#");
        let commit_message = match scope.is_empty() {
            true => format!("`{type}`: {sanitized_message}"),
            false => format!("`{type}` (`{scope}`): {sanitized_message}"),
        };
        let commit_command = if md_marker {
            format!("commit --message {commit_message}")
        } else {
            format!("commit --message {}", commit_message.replace('`', ""))
        };
        self.run_git_command(&format!("{commit_command} {args}"))
            .map_err(|e| e.into())
    }
}
