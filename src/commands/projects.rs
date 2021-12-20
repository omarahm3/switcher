use std::path::PathBuf;
use std::process::exit;
use crate::cli::ProgramInfo;
use crate::config;

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
