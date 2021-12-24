mod commands;
mod cli;
mod config;
mod git;

use crate::cli::get_program_info;
use crate::cli::CliCommand;
use crate::commands::projects;

/**
 * switcher project add <PROJECT_NAME> <|PROJECT_PATH>
 * switcher config <|detail|d>
 * switcher setup <PROJECT_NAME>
 * switcher branch <PROJECT_NAME> <BRANCH>
 */

fn main() {
    config::init();

    let program_info = get_program_info();

    match program_info.command {
        CliCommand::Help => println!("You want help!?"),
        CliCommand::Setup => projects::setup(program_info),
        CliCommand::Branch => projects::sync_projects(program_info),
        CliCommand::Config => config::print(program_info),
        CliCommand::Project => projects::check(program_info),
    }
}