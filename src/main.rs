mod cli;
mod commands;
mod config;
mod git;

use crate::cli::get_program_info;
use crate::cli::CliCommand;
use crate::commands::projects;

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
    switcher feature ./path-to-feature-file.json
    switcher config --detail
    switcher project remove example
    switcher version
");
}

fn print_version() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
    println!(
        "{} v{}
By: {}
Repoistory: {}",
        NAME, VERSION, AUTHORS, REPOSITORY
    );
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
        CliCommand::Version => print_version(),
        CliCommand::Feature => projects::sync_feature(program_info),
    }
}
