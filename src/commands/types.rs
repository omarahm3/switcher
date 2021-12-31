use crate::cli::ProgramInfo;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
pub struct AddProject {
    pub name: String,
    pub path: PathBuf,
}

impl AddProject {
    pub fn init(project_info: &ProjectInfo) -> AddProject {
        let args = &project_info.args;

        // TODO This is needed because if the command was passed with empty arguments then it will
        // somehow escape the None arm above
        if args.is_empty() {
            println!("You must specify at least the name of the project");
            exit(1);
        }

        let name = match args.get(0) {
            None => panic!("Project name can't be empty"),
            Some(name) => name,
        };

        let path = match args.get(1) {
            None => project_info.cwd.clone(),
            Some(path) => PathBuf::from(path),
        };

        AddProject {
            name: name.to_string(),
            path,
        }
    }
}

pub struct SyncProjectBranch {
    pub name: String,
    pub branch: String,
}

impl SyncProjectBranch {
    pub fn init(program: &ProgramInfo) -> SyncProjectBranch {
        let args = &program.args;
        let args = match args {
            None => {
                println!("You must specify at least the name of the project");
                exit(1);
            }
            Some(args) => args,
        };

        if args.is_empty() {
            println!("You must specify project name and branch");
            exit(1);
        }

        let name = match args.get(0) {
            None => panic!("Project name can't be empty"),
            Some(name) => name,
        };

        let branch = match args.get(1) {
            None => panic!("You must enter branch name as second argument"),
            Some(branch) => branch,
        };

        SyncProjectBranch {
            name: name.to_string(),
            branch: branch.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ProjectInfo {
    pub sub_command: ProjectCommand,
    pub args: Vec<String>,
    pub cwd: PathBuf,
}

#[derive(Debug)]
pub enum ProjectCommand {
    Add,
    Remove,
    Help,
}

impl ProjectCommand {
    pub fn command(cmd: &str) -> ProjectCommand {
        match cmd {
            "help" => ProjectCommand::Help,
            "add" => ProjectCommand::Add,
            "remove" => ProjectCommand::Remove,
            _ => ProjectCommand::Help,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub repository: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureConfig {
    pub project: String,
    pub feature_specs: Vec<Feature>,
}
