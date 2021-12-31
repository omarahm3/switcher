pub fn print_help() {
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
