use std::fs;
use std::process::{Command, Output};


pub enum ProjectType {
    Websocket,
    RestApi,
    MongodbRepository,
    DesktopApp,
    Game,
}

pub fn generate_project(name: &str, project_type: ProjectType) -> Result<(), &'static str> {
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
    let output = Command::new("cargo").arg("new").arg(name).output();
    
    match output {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to create a basic rust project. Is cargo installed?"),
    }
}

fn generate_websocket_project(name: &str) -> Result<(), &'static str> {
    create_vanilla_project(name)?;
    
    todo!()
}
