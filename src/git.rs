use std::fs;
use std::path::PathBuf;
use std::process::{Command};

pub fn get_repositories(path: &PathBuf) -> Vec<PathBuf> {
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

pub fn sync_repositories_to_branch(repositories: &Vec<PathBuf>, branch: &String) {
    for repository in repositories {
        let repo_name = repository.file_name().unwrap().to_str().unwrap();

        println!("!> Syncing repository: [{}]", repo_name);

        println!("\t!> Running git fetch");
        git_fetch(repository.to_path_buf());

        println!("\t!> Running git checkout {}", &branch);
        git_checkout(repository.to_path_buf(), branch.to_string());
        println!("\n");
    }
}

fn build_command(command: &str) -> (String, Vec<&str>) {
    let mut parts = command.trim().split(" ").collect::<Vec<&str>>();
    let command = &parts.remove(0);
    let args = &parts;
    (command.to_string(), args.to_vec())
}

fn run_command(command: String, args: Vec<&str>, cwd: PathBuf) {
    let command = match Command::new(command)
        .current_dir(cwd)
        .args(args)
        .output() {
            Err(err) => panic!("Error running command [{}]", err),
            Ok(cmd) => cmd,
        };

    if !command.status.success() {
        let error = String::from_utf8_lossy(&command.stderr);
        let error = error.trim();
        let output = String::from_utf8_lossy(&command.stdout);
        let output = output.trim();

        println!("\tOutput> {}", output);
        println!("\tError> {}", error);
    }
}

fn git_checkout(repository: PathBuf, branch: String) {
    let command_string = format!("git checkout {}", branch);
    let (command, args) = build_command(&command_string);
    run_command(command, args, repository);
}

fn git_fetch(repository: PathBuf) {
    let command_string = "git fetch";
    let (command, args) = build_command(command_string);
    run_command(command, args, repository);
}

