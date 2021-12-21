use std::fs;
use std::path::PathBuf;

pub fn get_repositories(path: &PathBuf) -> Vec<PathBuf> {
    let mut repositories: Vec<PathBuf> = Vec::new();

    // TODO Must make sure that projects paths are directory the moment user enters them
    let paths = match fs::read_dir(path) {
        Err(_) => panic!("Error reading directory"),
        Ok(paths) => paths,
    };

    for path in paths {
        let entry = match path {
            Err(err) => panic!("Unknown error occurred reading path [{}]", err),
            Ok(path) => path,
        };
        let mut git_path = entry.path();
        git_path.push(".git");

        if git_path.exists() {
            repositories.push(entry.path());
        }
    }

    repositories
}
