use super::types::FeatureConfig;
use crate::commands::types::{AddProject, ProjectCommand, ProjectInfo, SyncProjectBranch};
use crate::config;
use crate::core::cli::ProgramInfo;
use crate::core::git;
use std::fs;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

fn projects_help() {
    println!("
This will let you control your projects by adding or removing them, please note that you'll need to run `switcher setup <PROJECT_NAME>` afterwards

Commands:
    add, remove

Examples:
    switcher project add example /optional/project/path # Pass project path
    switcher project add example # project path will be then CWD
    switcher project remove example
");
}

fn setup_help() {
    println!("
This will help you setup your project by specifying a project name, and then it will let you select project's repositories

Examples:
    switcher setup example
");
}

pub fn sync_feature(program: ProgramInfo) {
    let args = match program.args {
        None => {
            println!("You must specify a sub-command");
            exit(1);
        }
        Some(args) => args,
    };

    let feature_file = match args.first() {
        Some(arg) => arg,
        None => {
            println!("You must specify a sub-command");
            exit(1);
        }
    };

    let file_path = Path::new(&feature_file);

    if !file_path.exists() {
        println!(
            "Please enter a valid path, since this path [{}] does not exist",
            feature_file
        );
        exit(1);
    }

    let feature_config = read_feature_file(file_path);
    let mut config = config::get();
    let project = match config.get_project(&feature_config.project) {
        Some(project) => project,
        None => {
            println!("Cannot find project with name: [{}] please make sure that you added it (run 'switcher config -d' to make sure)", feature_config.project);
            exit(1);
        }
    };

    for feature in feature_config.feature_specs {
        let repository = match project.get_repository_by_name(&feature.repository) {
            Some(repository) => repository,
            None => {
                println!(
                    "Cannot find repository with this name [{}]",
                    feature.repository
                );
                exit(1);
            }
        };
        git::sync_repository_to_branch(repository, &feature.branch);
    }
}

pub fn check(program: ProgramInfo) {
    let args = match program.args {
        None => {
            println!("You must specify a sub-command");
            exit(1);
        }
        Some(args) => args,
    };

    let mut options = args.iter();
    let sub_command = match options.next() {
        None => "help",
        Some(sub_command) => sub_command,
    };
    let sub_command = ProjectCommand::command(sub_command);

    let args = options.as_slice();

    let project_info = ProjectInfo {
        sub_command,
        args: args.to_vec(),
        cwd: program.cwd,
    };

    match project_info.sub_command {
        ProjectCommand::Add => add(project_info),
        ProjectCommand::Remove => remove(project_info),
        ProjectCommand::Help => projects_help(),
    };
}

pub fn sync_projects(program: ProgramInfo) {
    let mut config = config::get();
    let params = SyncProjectBranch::init(&program);
    let project = match config.get_project(&params.name) {
        None => {
            println!(
                "Unable to find project [{}] please consider running 'add_project {} <|PATH>'",
                &params.name, &params.name
            );
            exit(1);
        }
        Some(project) => project,
    };
    let repositories = &project.repositories;
    git::sync_repositories_to_branch(repositories, &params.branch);
}

pub fn setup(program: ProgramInfo) {
    let mut config = config::get();

    let mut args = match program.args {
        None => {
            setup_help();
            exit(1);
        }
        Some(args) => args,
    };

    let project_name = match args.pop() {
        None => {
            setup_help();
            exit(1);
        }
        Some(name) => name,
    };

    let mut project = match config.get_project(&project_name) {
        None => {
            println!(
                "Unable to find project [{}] please consider running 'add_project {} <|PATH>'",
                &project_name, &project_name
            );
            exit(1);
        }
        Some(project) => project,
    };

    let repositories = git::get_repositories(&project.path);
    project.repositories = get_selected_repositories(repositories);

    config.save();
}

fn get_selected_repositories(repositories: Vec<PathBuf>) -> Vec<PathBuf> {
    println!("Found git repositories under your project path, please add needed repositories:");
    let mut selected_repos: Vec<PathBuf> = Vec::new();

    for repository in repositories.iter() {
        print!(
            "Add repository ({})? enter 'no' to ignore> ",
            repository.display()
        );

        stdout().flush().expect("Error flushing stdout");

        let mut answer = String::new();

        stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");

        if let "no" = answer.trim().to_lowercase().as_str() {
            continue;
        }

        selected_repos.push(repository.to_path_buf());
    }

    selected_repos
}

fn add(project_info: ProjectInfo) {
    let mut config = config::get();
    let params = AddProject::init(&project_info);

    if config.projects.iter().any(|elem| elem.name == params.name) {
        println!(
            "Project with name [{}] already exists, please change it",
            params.name
        );
        exit(1);
    }

    let project = config::Project::create(&params.name, &params.path);

    config.projects.push(project);
    config.save();
}

fn remove(project_info: ProjectInfo) {
    let mut config = config::get();
    let mut args = project_info.args;
    let project_name = match args.pop() {
        None => {
            println!("You must enter project name to remove");
            exit(1);
        }
        Some(project) => project,
    };

    let project_index = match config.projects.iter().position(|p| p.name == project_name) {
        None => {
            println!("Could't find a project with this name [{}]", project_name);
            exit(1);
        }
        Some(index) => index,
    };

    config.projects.remove(project_index);
    config.save();
}

fn read_feature_file(path: &Path) -> FeatureConfig {
    let mut file = match fs::File::open(path) {
        Err(err) => panic!("Error reading config file [{}]", err),
        Ok(file) => file,
    };

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Error reading config file content");

    // This will fail here if serde can't serialize config file content to Project
    match serde_json::from_str::<FeatureConfig>(&content) {
        Err(err) => panic!("Error serializing config file: [{}]", err),
        Ok(data) => data,
    }
}
