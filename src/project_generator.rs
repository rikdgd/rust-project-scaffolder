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
    
    
    fn parse_crates_string(crates_string: String) -> Result<Vec<RustCrates>, &'static str> {
        let cleaned_crates = crates_string.trim().to_string();
        let seperator = ',';
        
        let crate_numbers: Vec<&str> = cleaned_crates.split(seperator).collect();
        let mut crates_buffer: Vec<RustCrates> = Vec::new();
        
        for crate_nr in crate_numbers {
            if crate_nr.len() > 1 {
                return Err("Crates input string was incorrectly formatted.");
            }
            
            if let Ok(rust_crate) = RustCrates::from_input_str(crate_nr) {
                if !crates_buffer.contains(&rust_crate) {
                    crates_buffer.push(rust_crate);
                }
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
    use crate::rust_crates::RustCrates;

    use super::ProjectGenerator;

    // TODO: Write test for parse_crates_string
    #[test]
    fn parse_crates_string_test() {
        let correct_ordered_input = "2,3,4".to_string();
        let correct_unordered_input = "1,5,2".to_string();
        let correct_single_crate_input = "7".to_string();
        let correct_line_break_input = "1,2\n".to_string();
        let empty_input = "".to_string();
        let line_break_input = "\n".to_string();
        let repeat_crate_input = "1,2,3,2".to_string();
        
        let space_seperated_input = "1 3 4".to_string();
        
        
        
        let correct_ordered_result = ProjectGenerator::parse_crates_string(correct_ordered_input).unwrap();
        let correct_unordered_result = ProjectGenerator::parse_crates_string(correct_unordered_input).unwrap();
        let correct_single_crate_result = ProjectGenerator::parse_crates_string(correct_single_crate_input).unwrap();
        let correct_line_break_result = ProjectGenerator::parse_crates_string(correct_line_break_input).unwrap();
        let empty_input_result = ProjectGenerator::parse_crates_string(empty_input).unwrap();
        let line_break_result = ProjectGenerator::parse_crates_string(line_break_input).unwrap();
        let repeat_crate_result = ProjectGenerator::parse_crates_string(repeat_crate_input).unwrap();
        
        let space_seperated_result = ProjectGenerator::parse_crates_string(space_seperated_input);
        
        
        assert_eq!(correct_ordered_result, vec![RustCrates::Quote, RustCrates::Libc, RustCrates::Rand]);
        assert_eq!(correct_unordered_result, vec![RustCrates::Syn, RustCrates::Serde, RustCrates::Quote]);
        assert_eq!(correct_single_crate_result, vec![RustCrates::Bytes]);
        assert_eq!(correct_line_break_result, vec![RustCrates::Syn, RustCrates::Quote]);
        assert_eq!(empty_input_result, Vec::new());
        assert_eq!(line_break_result, Vec::new());
        
        if let Ok(crates) = space_seperated_result {
            panic!("incorrect user input did not generate an error.");
        }
    }
}
