use std::io::{Write, stdout, stdin};
use std::path::PathBuf;
use std::process::exit;
use crate::cli::ProgramInfo;
use crate::config;
use crate::git;

#[derive(Debug)]
struct AddProject {
    name: String,
    path: PathBuf
}

impl AddProject {
    fn init(program: &ProgramInfo) -> AddProject {
        let args = &program.args;
        let args = match args {
            // TODO Not sure why its not entering here?! or when it should enter here?
            None => {
                println!("You must specify at least the name of the project");
                exit(1);
            },
            Some(args) => args
        };

        // TODO This is needed because if the command was passed with empty arguments then it will
        // somehow escape the None arm above
        if args.len() == 0 {
            println!("You must specify at least the name of the project");
            exit(1);
        }

        let name = match args.get(0) {
            None => panic!("Project name can't be empty"),
            Some(name) => name,
        };

        let path = match args.get(1) {
            None => program.cwd.clone(),
            Some(path) => PathBuf::from(path),
        };

        AddProject {
            name: name.to_string(),
            path,
        }
    }
}

pub fn setup(program: ProgramInfo) {
    let mut config = config::get();

    let mut args = match program.args {
        None => panic!("You must enter project name"),
        Some(args) => args,
    };

    let project_name = match args.pop() {
        None => panic!("Unable to get project name"),
        Some(name) => name,
    };

    let mut project = match config.get_project(&project_name) {
        None => {
            println!("Unable to find project [{}] please consider runnnig 'add_project {} <|PATH>'", &project_name, &project_name);
            exit(1);
        },
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
        print!("Add repository ({})? enter 'no' to ignore> ", repository.display());

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

pub fn add(program: ProgramInfo) {
    let mut config = config::get();
    let params = AddProject::init(&program);

    if config.projects.iter().any(|elem| elem.name == params.name) {
        println!("Project with name [{}] already exists, please change it", params.name);
        exit(1);
    }

    let project = config::Project::create(&params.name, &params.path);

    config.projects.push(project);
    config.save();
}
