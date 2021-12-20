mod commands;
mod cli;
mod config;

use crate::cli::get_program_info;
use crate::cli::CliCommand;
use crate::commands::projects;

/**
 * switcher add project <PROJECT_NAME> <PATH> ----->> path is optional
 * switcher add project <PROJECT_NAME> ---->> then we just add current directory
 */

fn main() {
    config::init();

    let program_info = get_program_info();

    match program_info.command {
        CliCommand::Help => println!("You want help!?"),
        CliCommand::Add => projects::add(program_info),
        CliCommand::Config => config::print(),
    }
}
