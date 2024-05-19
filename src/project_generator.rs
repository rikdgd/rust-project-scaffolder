use crate::project_type::ProjectType;
use crate::rust_crates::RustCrates;
use crate::Config;
use std::fs::OpenOptions;
use std::process::Command;
use std::error::Error;



pub struct ProjectGenerator {
    path: String,
    project_type: ProjectType,
    additional_crates: Vec<RustCrates>,
}
impl ProjectGenerator {
    /// Tries to generate a project from config, if project type was invalid, generates a desktop app.
    pub fn from_config(config: Config) -> ProjectGenerator {
        let project_type = ProjectType::from_str(&config.target_project).unwrap_or_else(|_| {
            println!("Failed to get the project type, generating default: 'websocket'.");
            ProjectType::Websocket
        });
        
        let mut additional_crates: Vec<RustCrates> = Vec::new();
        if let Some(crates_string) = config.additional_crates {
            additional_crates = ProjectGenerator::parse_crates_string(crates_string).unwrap();
        }
        
        ProjectGenerator {
            path: config.path,
            project_type,
            additional_crates,
        }
    }
    
    
    fn parse_crates_string(crates_string: String) -> Result<Vec<RustCrates>, Box<dyn Error>> {
        let cleaned_crates = crates_string.trim().to_string();
        let seperator = ',';
        
        let crate_numbers: Vec<&str> = cleaned_crates.split(seperator).collect();
        let mut crates_buffer: Vec<RustCrates> = Vec::new();
        
        for crate_nr in crate_numbers {
            if let Ok(rust_crate) = RustCrates::from_input_str(crate_nr) {
                crates_buffer.push(rust_crate);
            }
        }
        
        Ok(crates_buffer)
    }
    
    
    pub fn generate_project(&self) -> Result<(), Box<dyn Error>> {    
        self.create_vanilla_project()?;
        self.append_required_crates()?;
        
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
    
    fn append_required_crates(&self) -> Result<(), Box<dyn Error>> {
        let required_crates = self.project_type.get_required_crates();
        let mut cargo_toml = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/Cargo.toml", self.path))?;
        
        for rust_crate in required_crates {
            rust_crate.append_import_to_file(&mut cargo_toml);
        }
        
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    // TODO: Write test for parse_crates_string
}
