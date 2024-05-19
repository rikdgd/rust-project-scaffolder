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
                crates_buffer.push(RustCrates::Pixels);
                crates_buffer.push(RustCrates::Winit);
                crates_buffer.push(RustCrates::WinitInputHelper);
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
                adjust_main_file(project_path, PIXEL_WINDOW_MAIN);
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

const PIXEL_WINDOW_MAIN: &str = r#"use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(_) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(_) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
"#;
