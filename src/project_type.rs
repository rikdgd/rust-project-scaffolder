use crate::rust_crates::RustCrates;
use std::fs::OpenOptions;



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
    
    #[allow(unused)]
    pub fn adjust_source_files(&self, project_path: &str) {
        match self {
            ProjectType::Websocket => {
                let mut cargo_toml = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("./{project_path}/Cargo.toml"))
                    .expect("Failed to open Cargo.toml file.");
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
}
