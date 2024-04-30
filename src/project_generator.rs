use crate::rust_crates::RustCrates;
use std::fs::File;
use std::process::Command;
use std::error::Error;


#[allow(unused)]
pub enum ProjectType {
    Websocket,
    RestApi,
    MongodbRepository,
    DesktopApp,
    Game,
}
impl ProjectType {
    pub fn get_required_crates(&self) -> Vec<RustCrates> {
        let mut crates_buffer: Vec<RustCrates> = Vec::new();
        
        match self {
            ProjectType::Websocket => {
                crates_buffer.push(RustCrates::Tungstenite);
            },
            ProjectType::RestApi => {
                crates_buffer.push(RustCrates::Rocket);
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
        
        crates_buffer
    }
}

pub fn generate_project(name: &str, project_type: ProjectType) -> Result<(), Box<dyn Error>> {
    match project_type {
        ProjectType::Websocket => {
            generate_websocket_project(name)
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

fn create_vanilla_project(name: &str) -> Result<(), &'static str> {
    let output = {
        Command::new("cargo")
        .arg("new").arg(name)
        .output()
    };
    
    match output {
        Ok(_) => Ok(()),
        Err(_) => {
            Err("Failed to create a basic rust project. Is cargo installed and added to PATH?")
        },
    }
}

fn generate_websocket_project(name: &str) -> Result<(), Box<dyn Error>> {
    create_vanilla_project(name)?;
    
    let required_packages = ProjectType::Websocket.get_required_crates();
    let mut cargo_toml = File::open(format!("./{name}/Cargo.toml"))?;
    
    for rust_crate in required_packages {
        rust_crate.append_import_to_file(&mut cargo_toml);
    }
    
    Ok(())
}
