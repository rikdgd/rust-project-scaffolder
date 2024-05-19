mod project_generator;
mod rust_crates;
mod project_type;
mod file_modification;

use std::env;
use std::io;
use std::error::Error;

use project_generator::ProjectGenerator;



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
            let generator = guided_setup().unwrap();
            generator.generate_project().expect("Failed to generate project based on the given input.");
        },
    }

}

fn guided_setup() -> Result<ProjectGenerator, Box<dyn Error>> {
    println!("
    #########################################\n
    ######## Rust project scaffolder ########\n
    #########################################\n"
    );
    
    let mut type_buffer = String::new();
    let mut path_buffer = String::new();
    
    println!("\n\nWhat type of project would you like to create? (enter 1 - 4)");
    println!("    1. websocket");
    println!("    2. REST-api");
    println!("    3. pixel-window");
    println!("    4. game");
    io::stdin().read_line(&mut type_buffer)?;
    
    println!("How should the project be called? (Note: you can also pass a full path, default is the current direcory.)\n");
    io::stdin().read_line(&mut path_buffer)?;
    
    println!("Generating project...");
    let config = Config::new(
        type_buffer.trim().to_string(), 
        path_buffer.trim().to_string()
    );
    println!("Creating project with the following settings:\n{:?}", config);
    
    Ok(ProjectGenerator::from_config(config))
}



#[derive(Debug)]
pub struct Config {
    pub target_project: String,
    pub path: String,
}

impl Config {
    fn new(target_project: String, path: String) -> Config {
        Config {
            target_project,
            path,
        }
    }
    
    
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
                
                if project_name.starts_with('.') || project_name.starts_with('/') {
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
