use crate::project_type::ProjectType;
use crate::Config;
use std::fs::OpenOptions;
use std::process::Command;
use std::error::Error;



pub struct ProjectGenerator {
    path: String,
    project_type: ProjectType,
}
impl ProjectGenerator {
    pub fn new(project_path: &str, project_type: ProjectType) -> ProjectGenerator {
        ProjectGenerator {
            path: String::from(project_path),
            project_type
        }
    }
    
    /// Tries to generate a project from config, if project type was invalid, generates a desktop app.
    pub fn from_config(config: Config) -> ProjectGenerator {
        let project_type = ProjectType::from_str(&config.target_project).unwrap_or_else(|_| {
            println!("Failed to get the project type, generating default: 'desktop app'.");
            ProjectType::DesktopApp
        });
        
        ProjectGenerator {
            path: config.path,
            project_type,
        }
    }
    
    
    pub fn generate_project(&self) -> Result<(), Box<dyn Error>> {      
        self.create_vanilla_project()?;
        
        let required_crates = self.project_type.get_required_crates();
        let mut cargo_toml = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/Cargo.toml", self.path))?;
        
        for rust_crate in required_crates {
            rust_crate.append_import_to_file(&mut cargo_toml);
        }
        
        self.project_type.adjust_source_files(&self.path);
        
        Ok(())
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
}
