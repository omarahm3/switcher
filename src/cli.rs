use std::env;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
pub enum CliCommand {
    Help,
    Config,
    Setup,
    Branch,
    Project,
    Version,
    Feature,
}

impl CliCommand {
    fn command(cmd: &str) -> CliCommand {
        match cmd {
            "help" => CliCommand::Help,
            "project" => CliCommand::Project,
            "config" => CliCommand::Config,
            "setup" => CliCommand::Setup,
            "branch" => CliCommand::Branch,
            "version" => CliCommand::Version,
            "feature" => CliCommand::Feature,
            _ => CliCommand::Help,
        }
    }
}

#[derive(Debug)]
pub struct ProgramInfo {
    pub command: CliCommand,
    pub cwd: PathBuf,
    pub args: Option<Vec<String>>,
}

pub fn get_program_info() -> ProgramInfo {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("Can't get current working directory"),
    };

    if args.is_empty() {
        // TODO should probably show help here
        println!("Please enter a valid command");
        exit(1);
    }

    let command = args.remove(0);

    let command = CliCommand::command(&command);

    ProgramInfo {
        command,
        args: Some(args),
        cwd,
    }
}
