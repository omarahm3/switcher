use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

const CONFIG_INIT: &str = r#"
{
    "projects": []
}
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

impl Project {
    pub fn create(name: &String, path: &PathBuf) -> Project {
        Project {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub projects: Vec<Project>
}

impl Config {
    pub fn save(&self) {
        let content = match serde_json::to_string(self) {
            Err(err) => panic!("Error deserializing config object: {}", err),
            Ok(content) => content,
        };
        let path = get_config_path();
        let mut file = match fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path) {
                Err(err) => panic!("Error occurred while opening config file: {}", err),
                Ok(file) => file,
            };

        match file.write_all(content.as_bytes()) {
            Err(err) => panic!("Error writing to config file: [{}]", err),
            Ok(_) => {}
        }
    }
}

pub fn init() {
    println!("config::init:: initializing config");

    let config_path = get_config_path();

    println!("Config path: {:?}", config_path);

    handle_config_file(&config_path);
}

pub fn get() -> Config {
    let config_path = get_config_path();
    read_config_file(config_path)
}

pub fn print() {
    let config = get();
    let projects = config.projects;

    println!("Projects:");
    for project in projects.iter() {
        let path = match project.path.to_str() {
            None => "N/A",
            Some(path) => path,
        };
        println!("\t{}\t\t{}", project.name, path);
    }
}

fn read_config_file(path: PathBuf) -> Config {
    let mut file = match fs::File::open(path) {
        Err(err) => panic!("Error reading config file [{}]", err),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error reading config file content");

    // This will fail here if serde can't serialize config file content to Project
    match serde_json::from_str::<Config>(&content) {
        Err(err) => panic!("Error serializing config file: [{}]", err),
        Ok(data) => data,
    }
}

fn handle_config_file(path: &PathBuf) {
    // Get parent directory path
    let parent_dir = match path.parent() {
        None => panic!("Error getting parent directory from path"),
        Some(parent) => parent.to_path_buf(),
    };

    // Check project directory
    if !path_exists(&parent_dir) {
        println!("Config directory doesn't exist, creating it");
        create_config_directory(parent_dir);
    };

    // Check config file
    if !path_exists(&path) {
        println!("Config doesn't exist, creating file");
        create_config_file(&path);
    }
}

fn path_exists(path: &PathBuf) -> bool {
    fs::metadata(path).is_ok()
}

fn create_config_file(path: &PathBuf) {
    // Create the actual config file
    let mut file = match fs::File::create(path) {
        Err(err) => panic!("Something happened while trying to create config file: [{}]", err),
        Ok(file) => file,
    };

    // Write an empty object to it
    match file.write_all(CONFIG_INIT.as_bytes()) {
        Err(err) => panic!("Error writing to config file: [{}]", err),
        Ok(_) => {}
    }
}

fn create_config_directory(path: PathBuf) {
    match fs::DirBuilder::new().recursive(true).create(path) {
        Err(err) => panic!("Error creating config directory: [{}]", err),
        Ok(_) => {}
    }
}

fn get_config_path() -> PathBuf {
    // Building config directory projects file path
    dirs::config_dir().map(|mut config| {
        config.push("switcher");
        config.push("config.json");
        config
    }).expect("Can't get config directory")
}
