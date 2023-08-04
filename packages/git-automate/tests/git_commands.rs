use miette::IntoDiagnostic;
use std::process::Output;

use git_automate::*;

// * Typesafe mutable static variables to reliably handle concurrency
static TEST_MUTEX: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
static TEST_LEN: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();

fn count_success_or_finish() -> i32 {
    let all_tests = 12;

    let test_len = TEST_LEN.get_or_init(|| std::sync::Mutex::new(0));
    let mut value_guard = match test_len.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    *value_guard += 1;
    if *value_guard == all_tests {
        println!("\t✅ SUCCESS! ALL {} TESTS PASSED", *value_guard);
    }
    *value_guard
}

fn prepare_test(dir_name: &str) -> miette::Result<String> {
    let test_mutex = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(()));

    let _guard = match test_mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    miette::set_panic_hook();
    let tmp_dir = std::env::temp_dir();

    // * Create a timestamp with its thread id to append to the directory name
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get UNIX Epoch timestamp")
        .as_nanos();
    let thread_id = std::thread::current().id();

    // * Initialize a new Git repository with the timestamp for each thread
    let mut repository = tmp_dir.clone();
    repository.push(format!("{dir_name}_{timestamp}_{thread_id:?}"));
    let repository = repository
        .to_str()
        .expect("Failed to create repository with timestamp");
    let init = git_command_tester(&["init", repository]).expect("Failed to run git init");
    assert!(init.status.success(), "Failed to git init");

    // * Change the directory to the initialized repository and create a new file
    std::env::set_current_dir(repository)
        .expect("Failed to change the current directory to repository");
    let file_name = "new_file.txt";
    let output = std::fs::write(file_name, "Hello world!");
    assert!(output.is_ok(), "Failed to write file");

    let stage_files = git_command_tester(&["add", "."]).expect("Failed to add files to staging");

    assert!(
        stage_files.status.success(),
        "Failed to add files to staging"
    );
    Ok(repository.to_string())
}

fn git_command_tester(args: &[&str]) -> miette::Result<Output> {
    std::process::Command::new("git")
        .args(args)
        .output()
        .into_diagnostic()
}

mod empty_repository {
    use super::*;
    #[test]
    fn init_repository() -> miette::Result<()> {
        prepare_test("test_run_init_repository")?;
        let test_dir = std::env::current_dir()
            .into_diagnostic()?
            .display()
            .to_string();
        println!("[init_repository]\nInitialize git repository at: {test_dir}");
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn status_without_commits() -> miette::Result<()> {
        prepare_test("test_run_status_without_commits")?;
        let git_cmd = GitCommand::new();
        let status = git_cmd.git_status("")?.stdout_to_string()?;
        assert!(status.contains("No commits yet"));
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn log_return_empty() -> miette::Result<()> {
        prepare_test("test_run_log_return_empty")?;
        let git_cmd = GitCommand::new();
        let log = git_cmd.git_log("")?.stdout_to_string()?;
        assert!(log.is_empty());
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn diff_return_empty() -> miette::Result<()> {
        prepare_test("test_run_diff_return_empty")?;
        let git_cmd = GitCommand::new();
        let diff = git_cmd.git_diff("")?.stdout_to_string()?;
        assert_eq!(diff, "");
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn staging_return_empty() -> miette::Result<()> {
        prepare_test("test_run_staging_return_empty")?;
        let git_cmd = GitCommand::new();
        let staging = git_cmd.git_staging_area(".")?.stdout_to_string()?;
        assert_eq!(staging, "");
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn stash_return_empty() -> miette::Result<()> {
        prepare_test("test_run_stash_return_empty")?;
        let git_cmd = GitCommand::new();
        let stash = git_cmd.git_stash("")?.stdout_to_string()?;
        assert_eq!(stash, "");
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn branch_return_empty() -> miette::Result<()> {
        prepare_test("test_run_branch_return_empty")?;
        let git_cmd = GitCommand::new();
        let branch = git_cmd.git_branch("")?.stdout_to_string()?;
        assert_eq!(branch, "");
        count_success_or_finish();
        Ok(())
    }
}

mod commit_initial_message {
    use super::*;
    #[test]
    fn init_repository() -> miette::Result<()> {
        prepare_test("test_run_init_repository")?;
        let test_dir = std::env::current_dir()
            .into_diagnostic()?
            .display()
            .to_string();
        println!("[commit_initial_message]\nInitialize git repository at: {test_dir}");
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn simple_commit() -> miette::Result<()> {
        prepare_test("test_run_simple_commit")?;
        let git_cmd = GitCommand::new();
        let commit = git_cmd
            .git_simple_commit("This commit with has safe use of spaces", "--dry-run") // note1: --dry-run bypass the read-only GitHub Actions
            .into_diagnostic()?
            .stdout_to_string()?;
        assert!(commit.contains("Initial commit")); // note2: --dry-run defaults to 'Initial commit' message
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn rename_as_main_branch() -> miette::Result<()> {
        prepare_test("test_run_rename_as_main_branch")?;
        let git_cmd = GitCommand::new();
        let _branch = git_cmd
            .git_branch("--move master main")?
            .stdout_to_string()?;
        let branch = git_cmd.git_branch("--show-current")?.stdout_to_string()?;
        assert_eq!(branch, "main\n");
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn check_git_status() -> miette::Result<()> {
        let curr_path = prepare_test("test_run_check_git_status")?;
        let file = "main.rs";
        let mut file_path = std::path::PathBuf::from(curr_path);
        file_path.push(file);

        std::fs::write(file_path, "fn main() {}").expect("Failed to write to file");

        let git_cmd = GitCommand::new();
        let status = git_cmd
            .git_status("--untracked-files")?
            .stdout_to_string()?;
        assert!(status.contains(file));
        count_success_or_finish();
        Ok(())
    }
    #[test]
    fn check_git_diff() -> miette::Result<()> {
        prepare_test("test_run_check_git_diff")?;
        let git_cmd = GitCommand::new();
        let diff = git_cmd.git_diff("--staged")?.stdout_to_string()?;
        let file = "new_file.txt";
        assert!(diff.contains(file));
        count_success_or_finish();
        Ok(())
    }
}
