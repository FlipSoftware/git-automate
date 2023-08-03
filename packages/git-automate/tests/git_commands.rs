use miette::IntoDiagnostic;
use std::process::Output;

use git_automate::*;

// * Typesafe mutable static variables to reliably handle concurrency
static TEST_MUTEX: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
static TEST_LEN: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();

fn count_success_or_clean_on_finish() -> i32 {
    let all_tests = 12;

    let test_len = TEST_LEN.get_or_init(|| std::sync::Mutex::new(0));
    let mut value_guard = match test_len.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    *value_guard += 1;
    if *value_guard == all_tests {
        println!("\t✅ SUCCESS! ALL {} TESTS PASSED", *value_guard);
        clean_up().unwrap();
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

fn clean_up() -> miette::Result<()> {
    let temp_dir = std::env::temp_dir().display().to_string();
    println!("\t♻ Cleaning directories");
    Ok(())
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
        println!("[init_repository] Initialize git repository at: {test_dir}");
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn status_without_commits() -> miette::Result<()> {
        prepare_test("test_run_status_without_commits")?;
        let status = git_status(&[]).expect("Failed to run git status");
        assert!(status.contains("No commits yet"));
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn log_return_empty() -> miette::Result<()> {
        prepare_test("test_run_log_return_empty")?;
        let log = git_log(&[]).expect("Failed to run git log");
        assert!(log.is_empty());
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn diff_return_empty() -> miette::Result<()> {
        prepare_test("test_run_diff_return_empty")?;
        let diff = git_diff(&[]).expect("Failed to run git diff");
        assert_eq!(diff, "");
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn staging_return_empty() -> miette::Result<()> {
        prepare_test("test_run_staging_return_empty")?;
        let staging = git_staging_area(&["."]).expect("Failed to add files to staging");
        assert_eq!(staging, "");
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn stash_return_empty() -> miette::Result<()> {
        prepare_test("test_run_stash_return_empty")?;
        let stash = git_stash(&[]).expect("Failed to run git stash");
        assert_eq!(stash, "");
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn branch_return_empty() -> miette::Result<()> {
        prepare_test("test_run_branch_return_empty")?;
        let branch = git_branch(&[]).expect("Failed to run git branch");
        assert_eq!(branch, "");
        count_success_or_clean_on_finish();
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
        println!("[commit_initial_message] Initialize git repository at: {test_dir}");
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn simple_commit() -> miette::Result<()> {
        prepare_test("test_run_simple_commit")?;
        let commit = git_simple_commit(&["Initial commit"]).expect("Failed to run simple commit");
        assert!(commit.contains("Initial commit"));
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn rename_as_main_branch() -> miette::Result<()> {
        prepare_test("test_run_rename_as_main_branch")?;
        let _branch =
            git_branch(&["--move", "master", "main"]).expect("Failed to run git branch --move");
        let branch = git_branch(&["--show-current"]).expect("Failed to run git status");
        assert_eq!(branch, "main\n");
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn check_git_status() -> miette::Result<()> {
        let curr_path = prepare_test("test_run_check_git_status")?;
        let file = "main.rs";
        let mut file_path = std::path::PathBuf::from(curr_path);
        file_path.push(file);

        std::fs::write(file_path, "fn main() {}").expect("Failed to write to file");

        let status = git_status(&["--untracked-files"]).expect("Failed to run git status");
        assert!(status.contains(file));
        count_success_or_clean_on_finish();
        Ok(())
    }
    #[test]
    fn check_git_diff() -> miette::Result<()> {
        prepare_test("test_run_check_git_diff")?;
        let diff = git_diff(&["--staged"]).expect("Failed to run git diff --staged");
        let file = "new_file.txt";
        assert!(diff.contains(file));
        count_success_or_clean_on_finish();
        Ok(())
    }
}
