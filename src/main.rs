mod commands;
mod core;

use crate::core::cli::get_program_info;
use crate::core::cli::CliCommand;
use crate::core::config;
use commands::all;
use commands::help;
use commands::projects;
use commands::version;

fn main() {
    config::init();

    let program_info = get_program_info();

    match program_info.command {
        CliCommand::Help => help::print_help(),
        CliCommand::Setup => projects::setup(program_info),
        CliCommand::Branch => projects::sync_projects(program_info),
        CliCommand::Config => config::print(program_info),
        CliCommand::Project => projects::check(program_info),
        CliCommand::Version => version::print_version(),
        CliCommand::Feature => projects::sync_feature(program_info),
        CliCommand::All => all::run_git(program_info),
    }
}
