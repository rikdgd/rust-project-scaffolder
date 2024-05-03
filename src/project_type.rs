use crate::rust_crates::RustCrates;
use std::fs::{File, OpenOptions};
use std::io::Write;



const TUNGSTENITE_MAIN: &str = r#"use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

/// A WebSocket echo server
fn main () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}
"#;

const ROCKET_MAIN: &str = r#"#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
"#;



#[allow(unused)]
pub enum ProjectType {
    Websocket,
    RestApi,
    MongodbRepository,
    DesktopApp,
    Game,
}
impl ProjectType {
    pub fn from_str(type_str: &str) -> Result<ProjectType, &'static str> {
        match type_str {
            "websocket" => Ok(ProjectType::Websocket),
            "restapi" => Ok(ProjectType::RestApi),
            "mongorepo" => Ok(ProjectType::MongodbRepository),
            "desktop" => Ok(ProjectType::DesktopApp),
            "game" => Ok(ProjectType::Game),
            _ => Err("Provided project type is incorrect."),
        }
    }
    
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
        println!("Adjusting source file for: '{project_path}'");
        
        let adjust_main_file = |project_path: &str, new_content: &str| {
            let mut main_rs = OpenOptions::new()
                .write(true)
                .append(true)
                .open(format!("{project_path}/src/main.rs"))
                .expect("Failed to open main.rs file."); 
            
            main_rs.set_len(0).expect("Failed to clear the main.rs file.");
            main_rs.write_all(new_content.as_bytes())
                .expect("Failed to adjust main.rs file.");
        };
        
        match self {
            ProjectType::Websocket => {
                adjust_main_file(project_path, TUNGSTENITE_MAIN);
            },
            ProjectType::RestApi => {
                adjust_main_file(project_path, ROCKET_MAIN);
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
