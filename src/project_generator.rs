use crate::project_type::ProjectType;
use std::fs::OpenOptions;
use std::process::Command;
use std::error::Error;



pub struct ProjectGenerator {
    path: String,
}
impl ProjectGenerator {
    pub fn new(project_path: &str) -> ProjectGenerator{
        ProjectGenerator {
            path: String::from(project_path),
        }
    }
    
    pub fn generate_project(&self, project_type: ProjectType) -> Result<(), Box<dyn Error>> {
        match project_type {
            ProjectType::Websocket => {
                self.generate_websocket_project()
            },
            ProjectType::RestApi => {
                todo!()
            },
            ProjectType::MongodbRepository => {
                todo!()
            },
            ProjectType::DesktopApp => {
                todo!()
            },
            ProjectType::Game => {
                todo!()
            },
        }
    }
    
    fn create_vanilla_project(&self) -> Result<(), &'static str> {
        let output = {
            Command::new("cargo")
            .arg("new").arg(&self.path)
            .output()
        };
        
        match output {
            Ok(_) => Ok(()),
            Err(_) => {
                Err("Failed to create a basic rust project. Is cargo installed and added to PATH?")
            },
        }
    }
    
    fn generate_websocket_project(&self) -> Result<(), Box<dyn Error>> {
        self.create_vanilla_project()?;
        
        let required_packages = ProjectType::Websocket.get_required_crates();
        let mut cargo_toml = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/Cargo.toml", self.path))
            .expect("Failed to open Cargo.toml file.");
        
        for rust_crate in required_packages {
            rust_crate.append_import_to_file(&mut cargo_toml);
        }
        
        Ok(())
    }
}
