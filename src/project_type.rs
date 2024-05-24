use crate::rust_crates::RustCrates;
use crate::file_modification::*;



#[allow(unused)]
pub enum ProjectType {
    Websocket,
    RestApi,
    Game,
}
impl ProjectType {
    pub fn from_str(type_str: &str) -> Result<ProjectType, &'static str> {
        match type_str {
            "1" => Ok(ProjectType::Websocket),
            "websocket" => Ok(ProjectType::Websocket),
            
            "2" => Ok(ProjectType::RestApi),
            "restapi" => Ok(ProjectType::RestApi),
            
            "3" => Ok(ProjectType::Game),
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
            ProjectType::Game => {
                crates_buffer.push(RustCrates::Macroquad);
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
            ProjectType::Game => {
                adjust_main_file(project_path, MACROQUAD_MAIN);
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


const MACROQUAD_MAIN: &str = r#"use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}"#;
