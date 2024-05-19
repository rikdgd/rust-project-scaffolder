use crate::rust_crates::RustCrates;
use crate::file_modification::*;



// TODO: DesktopApp
#[allow(unused)]
pub enum ProjectType {
    Websocket,
    RestApi,
    PixelWindow,
    Game,
}
impl ProjectType {
    pub fn from_str(type_str: &str) -> Result<ProjectType, &'static str> {
        match type_str {
            "websocket" => Ok(ProjectType::Websocket),
            "restapi" => Ok(ProjectType::RestApi),
            "pixelwindow" => Ok(ProjectType::PixelWindow),
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
            ProjectType::PixelWindow => {
                todo!()
            },
            ProjectType::Game => {
                crates_buffer.push(RustCrates::Bevy);
            },
        }
        
        crates_buffer
    }
    
    pub fn adjust_source_files(&self, project_path: &str) {
        println!("Adjusting source file for: '{project_path}'");
        
        match self {
            ProjectType::Websocket => {
                adjust_main_file(project_path, TUNGSTENITE_MAIN);
            },
            ProjectType::RestApi => {
                adjust_main_file(project_path, ROCKET_MAIN);
            },
            ProjectType::PixelWindow => {
                todo!()
            },
            ProjectType::Game => {
                adjust_main_file(project_path, BEVY_MAIN);
                append_to_cargo_toml(project_path, BEVY_OPTIMIZATION);
            },
        }        
    }
    
    
}



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


const BEVY_MAIN: &str = r#"use bevy::prelude::*;

fn main(){
  App::new()
    .add_plugins(DefaultPlugins)
    .run();
}"#;


const BEVY_OPTIMIZATION: &str = r#"

# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
"#;
