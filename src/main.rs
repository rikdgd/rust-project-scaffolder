mod project_generator;
mod rust_crates;
mod project_type;

use std::env;

use project_generator::ProjectGenerator;
use project_type::ProjectType;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(args);
    
    match config {
        Some(config) => {
            println!("Config:\n{:?}", config);
            let generator = ProjectGenerator::from_config(config);
            generator.generate_project().expect("Failed to generate project from config.");
        },
        None => {
            println!("No arguments provided, starting guided setup...");
            
            // testing
            let generator = ProjectGenerator::new(
                "./generated-project", 
                ProjectType::Websocket,
            );
            generator.generate_project().expect("Failed to generate testing project");
        },
    }

}



#[derive(Debug)]
pub struct Config {
    pub target_project: String,
    pub path: String,
}

impl Config {
    fn from_args(args: Vec<String>) -> Option<Config> {
        match args.len() {
            1 => None, 
            2 => {
                Some(Config {
                    target_project: String::from(&args[1]),
                    path: format!("./new-{}-project", &args[1]),
                })
            },
            3 => {
                let target_project = &args[1];
                let project_name = &args[2];
                
                if project_name.starts_with(".") || project_name.starts_with("/") {
                    Some(Config { 
                        target_project: target_project.clone(),
                        path: project_name.clone(),
                    })
                } else {
                    Some(Config {
                        target_project: target_project.clone(),
                        path: format!("./{}", project_name),
                    })
                }
            }
            _ => None,
        }
    }
}
