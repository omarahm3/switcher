use colored::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn get_repositories(path: &Path) -> Vec<PathBuf> {
    let mut repositories: Vec<PathBuf> = Vec::new();

    // TODO Must make sure that projects paths are directory the moment user enters them
    let paths = match fs::read_dir(path) {
        Err(_) => panic!("Error reading directory"),
        Ok(paths) => paths,
    };

    for path in paths {
        let entry = match path {
            Err(err) => panic!("Unknown error occurred reading path [{}]", err),
            Ok(path) => path,
        };
        let mut git_path = entry.path();
        git_path.push(".git");

        if git_path.exists() {
            repositories.push(entry.path());
        }
    }

    repositories
}

pub fn git_current_branch(repository: &Path) -> String {
    let command_string = "git rev-parse --abbrev-ref HEAD";
    let (command, args) = build_command(command_string);
    let (error, output) = run_command(command, args, repository);

    if !error.is_empty() {
        println!("Error getting current branch of repository: [{}]", error);
    }

    output
}

pub fn sync_repository_to_branch(repository: PathBuf, branch: &str) {
    let repo_name = repository.file_name().unwrap().to_str().unwrap();

    println!("!> Syncing repository: [{}]", repo_name);

    println!("\t!> Running git fetch");
    git_fetch(&repository);

    let should_stash = git_has_changes(&repository);

    if should_stash {
        println!("\t!> Running git stash");
        git_stash(&repository);
    }

    println!("\t!> Running git checkout {}", &branch);
    let (err, _) = git_checkout(&repository, branch.to_string());

    if !err.is_empty() && err.contains("did not match any file(s) known to git") {
        println!("\t{}", "!!> Branch does not exist on this repository".red());
    }

    println!("\t!> Running git pull");
    let (err, _) = git_pull(&repository);

    if !err.is_empty() && err.contains("but no such ref was fetched") {
        println!(
            "\t{}",
            "!!> Branch does not exist on the remote, most probably remote branch was removed"
                .red()
        );
    }

    println!("\n");
}

pub fn sync_repositories_to_branch(repositories: &[PathBuf], branch: &str) {
    for repository in repositories {
        sync_repository_to_branch(repository.to_path_buf(), branch)
    }
}

fn build_command(command: &str) -> (String, Vec<&str>) {
    let mut parts = command.trim().split(' ').collect::<Vec<&str>>();
    let command = &parts.remove(0);
    let args = &parts;
    (command.to_string(), args.to_vec())
}

pub fn run_command(command: String, args: Vec<&str>, cwd: &Path) -> (String, String) {
    let command = match Command::new(command).current_dir(cwd).args(args).output() {
        Err(err) => panic!("Error running command [{}]", err),
        Ok(cmd) => cmd,
    };

    let error = String::from_utf8_lossy(&command.stderr);
    let error = error.trim().to_string();
    let output = String::from_utf8_lossy(&command.stdout);
    let output = output.trim().to_string();

    (error, output)
}

fn git_checkout(repository: &Path, branch: String) -> (String, String) {
    let command_string = format!("git checkout {}", branch);
    let (command, args) = build_command(&command_string);
    run_command(command, args, repository)
}

fn git_pull(repository: &Path) -> (String, String) {
    let (command, args) = build_command("git pull");
    run_command(command, args, repository)
}

fn git_fetch(repository: &Path) -> (String, String) {
    let command_string = "git fetch";
    let (command, args) = build_command(command_string);
    run_command(command, args, repository)
}

fn git_stash(repository: &Path) -> (String, String) {
    let command_string = "git stash save 'switcher:: changes'";
    let (command, args) = build_command(command_string);
    run_command(command, args, repository)
}

fn git_has_changes(repository: &Path) -> bool {
    let command_string = "git status -s";
    let (command, args) = build_command(command_string);
    let (_, output) = run_command(command, args, repository);

    !output.is_empty()
}
