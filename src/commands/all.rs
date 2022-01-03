use colored::*;
use std::process::exit;

use crate::core::{cli::ProgramInfo, config, git};

fn help() {
    println!(
        "
This will let you run git commands on all of your registered project repositories

Examples:
    switcher all help
    switcher all <PROJECT_NAME> stash clear
"
    );
}

pub fn run_git(program: ProgramInfo) {
    let args = match program.args {
        None => {
            println!("Command must be: 'switcher all <PROJECT_INFO> <GIT_COMMAND>' run 'help' to check examples");
            exit(1);
        }
        Some(args) => args,
    };

    let mut options = args.iter();
    let project_name = match options.next() {
        None => "help",
        Some(sub_command) => sub_command,
    };

    if project_name == "help" {
        help();
        exit(0);
    }

    println!("Project Name: {}", project_name);

    let git_command = options.as_slice().to_vec();

    if git_command.is_empty() {
        println!("Command must be: 'switcher all <PROJECT_INFO> <GIT_COMMAND>' run 'help' to check examples");
        exit(1);
    }
    let command_str: String = git_command.join(" ");
    // Convert Vec<String> to Vec<&str> for git::run_command
    let git_command: Vec<&str> = git_command.iter().map(String::as_ref).collect();

    let mut config = config::get();
    let project = match config.get_project(project_name) {
        None => {
            println!(
                "Unable to find project [{}] please consider running 'add_project {} <|PATH>'",
                &project_name, &project_name
            );
            exit(1);
        }
        Some(project) => project,
    };

    let repositories = &project.repositories;

    for repository in repositories {
        println!("!> Running {}", command_str);

        let (err, output) = git::run_command("git".to_string(), git_command.to_vec(), repository);

        if !err.is_empty() {
            println!("{}", err.red());
        }

        if !output.is_empty() {
            println!("{}", output.blue());
        }
    }
}
