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
            additional_crates = ProjectGenerator::parse_crates_string(crates_string);
        }
        
        ProjectGenerator {
            path: config.path,
            project_type,
            additional_crates,
        }
    }
    
    
    fn parse_crates_string(crates_str: String) -> Vec<RustCrates> {
        let separator = ',';
        let crates_vec: Vec<&str> = crates_str
            .trim()
            .split(separator)
            .collect();
        
        let mut crates_buffer: Vec<RustCrates> = Vec::new();
        for crate_str in crates_vec {
            if crate_str.is_empty() {
                continue;
            }

            if let Ok(rust_crate) = RustCrates::from_input_str(crate_str) {
                if !crates_buffer.contains(&rust_crate) {
                    crates_buffer.push(rust_crate);
                }
            }
        }
        
        crates_buffer
    }
    
    
    pub fn generate_project(&self) -> Result<(), Box<dyn Error>> {
        match self.project_type {
            ProjectType::ProcMacro => {
                self.create_vanilla_project(true)?;
            },
            _ => {
                self.create_vanilla_project(false)?;
            }
        }
        self.append_required_crates()?;
        
        self.project_type.adjust_source_files(&self.path);
        
        Ok(())
    }
    
    fn create_vanilla_project(&self, is_lib: bool) -> Result<(), &'static str> {
        let project_type_arg = match is_lib {
            true => "--lib",
            false => "--bin",
        };
        let output = {
            Command::new("cargo")
            .arg("new").arg(&self.path).arg(project_type_arg)
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
        let mut required_crates = self.project_type.get_required_crates();
        for rust_crate in &self.additional_crates {
            required_crates.push(*rust_crate);
        }
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
    use crate::rust_crates::RustCrates;
    use super::ProjectGenerator;

    #[test]
    fn parse_crates_string_test() {
        // Should succeed
        let correct_ordered_input = "2,3,4".to_string();
        let correct_unordered_input = "1,5,2".to_string();
        let correct_single_crate_input = "7".to_string();
        let correct_line_break_input = "1,2\n".to_string();
        let empty_input = "".to_string();
        let line_break_input = "\n".to_string();
        let repeat_crate_input = "1,2,3,2".to_string();
        let non_number_input = "t,w,y,3,f,d,2".to_string();
        
        // Should return Err
        let space_seperated_input = "1 3 4".to_string();
        let random_input = "hdvfn nvfn  988 8fhs\nf".to_string();
        
        
        
        let correct_ordered_result = ProjectGenerator::parse_crates_string(correct_ordered_input);
        let correct_unordered_result = ProjectGenerator::parse_crates_string(correct_unordered_input);
        let correct_single_crate_result = ProjectGenerator::parse_crates_string(correct_single_crate_input);
        let correct_line_break_result = ProjectGenerator::parse_crates_string(correct_line_break_input);
        let repeat_crate_result = ProjectGenerator::parse_crates_string(repeat_crate_input);
        let empty_input_result = ProjectGenerator::parse_crates_string(empty_input);
        let line_break_result = ProjectGenerator::parse_crates_string(line_break_input);
        let non_number_result = ProjectGenerator::parse_crates_string(non_number_input);
        
        let space_seperated_result = ProjectGenerator::parse_crates_string(space_seperated_input);
        let random_result = ProjectGenerator::parse_crates_string(random_input);
        
        
        
        assert_eq!(correct_ordered_result, vec![RustCrates::Quote, RustCrates::Libc, RustCrates::Rand]);
        assert_eq!(correct_unordered_result, vec![RustCrates::Syn, RustCrates::Serde, RustCrates::Quote]);
        assert_eq!(correct_single_crate_result, vec![RustCrates::Bytes]);
        assert_eq!(correct_line_break_result, vec![RustCrates::Syn, RustCrates::Quote]);
        assert_eq!(repeat_crate_result, vec![RustCrates::Syn, RustCrates::Quote, RustCrates::Libc]);
        assert_eq!(empty_input_result, Vec::new());
        assert_eq!(line_break_result, Vec::new());
        assert_eq!(non_number_result, vec![RustCrates::Libc, RustCrates::Quote]);
        
        assert_eq!(space_seperated_result, Vec::new());
        assert_eq!(random_result, Vec::new());
    }
}
