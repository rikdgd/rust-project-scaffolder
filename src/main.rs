mod project_generator;
mod popular_crates;

use std::env;

use project_generator::{generate_project, ProjectType};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(args);
    
    match config {
        Some(config) => {
            println!("Target project: {}", config.target_project);
        },
        None => {
            println!("Wrong amount of arguments provided!");
        },
    }
    
    // testing
    generate_project("generated-project", ProjectType::Websocket)
        .expect("Failed to generate project");
}



struct Config {
    target_project: String,
}

impl Config {
    fn from_args(args: Vec<String>) -> Option<Config> {
        match args.len() {
            1 => None, 
            2 => Some(Config {target_project: String::from(&args[1])}),
            _ => None,
        }
    }
}
