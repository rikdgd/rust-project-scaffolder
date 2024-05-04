use std::fs::OpenOptions;
use std::io::Write;



pub fn adjust_main_file(project_path: &str, new_content: &str) {
    let mut main_rs = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{project_path}/src/main.rs"))
        .expect("Failed to open main.rs file.");
    
    main_rs.set_len(0).expect("Failed to clear the main.rs file.");
    main_rs.write_all(new_content.as_bytes())
        .expect("Failed to adjust main.rs file.");
}

pub fn append_to_cargo_toml(project_path: &str, new_content: &str) {
    let mut cargo_toml = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/Cargo.toml", project_path))
        .expect("Failed to open 'Cargo.toml' file.");
    
    cargo_toml.write_all(new_content.as_bytes()).expect("Failed to write to 'Cargo.toml' file.");
}
