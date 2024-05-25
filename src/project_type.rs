use crate::rust_crates::RustCrates;
use crate::file_modification::*;



#[allow(unused)]
pub enum ProjectType {
    ProcMacro,
    Websocket,
    RestApi,
    Game,
}
impl ProjectType {
    pub fn from_str(type_str: &str) -> Result<ProjectType, &'static str> {
        match type_str {
            "1" => Ok(Self::ProcMacro),
            "proc-macro" => Ok(Self::ProcMacro),
            
            "2" => Ok(ProjectType::Websocket),
            "websocket" => Ok(ProjectType::Websocket),
            
            "3" => Ok(ProjectType::RestApi),
            "restapi" => Ok(ProjectType::RestApi),
            
            "4" => Ok(ProjectType::Game),
            "game" => Ok(ProjectType::Game),
            
            _ => Err("Provided project type is incorrect."),
        }
    }
    
    pub fn get_required_crates(&self) -> Vec<RustCrates> {
        let mut crates_buffer: Vec<RustCrates> = Vec::new();
        
        match self {
            ProjectType::ProcMacro => {
                crates_buffer.push(RustCrates::Syn);
                crates_buffer.push(RustCrates::Quote);
            },
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
        let adjust_main_err = "Failed to adjust main.rs/lib.rs file.";
        println!("Adjusting source file for: '{project_path}'");
        
        match self {
            ProjectType::ProcMacro => {
                adjust_main_file(project_path, PROC_MACRO_MAIN, true).expect(adjust_main_err);
                adjust_proc_macro(project_path).expect("Failed to adjust Cargo.toml file.");
            },
            ProjectType::Websocket => {
                adjust_main_file(project_path, TUNGSTENITE_MAIN, false).expect(adjust_main_err);
            },
            ProjectType::RestApi => {
                adjust_main_file(project_path, ROCKET_MAIN, false).expect(adjust_main_err);
            },
            ProjectType::Game => {
                adjust_main_file(project_path, MACROQUAD_MAIN, false).expect(adjust_main_err);
            },
        }        
    }
    
    
}



const PROC_MACRO_MAIN: &str = r#"use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
"#;


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
