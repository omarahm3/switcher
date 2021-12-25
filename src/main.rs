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

fn print_help() {
    println!("
Switcher is a simple organizer for projects with multi-repositories gives you the ability to do bulk git actions on all of them

Usage:
    switcher <COMMAND> <SUB_COMMAND>

Examples:
    switcher project add example /optional/project/path # Pass project path
    switcher project add example # project path will be then CWD
    switcher setup example
    switcher branch example develop
    switcher config --detail
    switcher project remove example
");
}

fn main() {
    config::init();

    let program_info = get_program_info();

    match program_info.command {
        CliCommand::Help => print_help(),
        CliCommand::Setup => projects::setup(program_info),
        CliCommand::Branch => projects::sync_projects(program_info),
        CliCommand::Config => config::print(program_info),
        CliCommand::Project => projects::check(program_info),
    }
}
