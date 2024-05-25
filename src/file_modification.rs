use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Write};



pub fn adjust_main_file(project_path: &str, new_content: &str, is_lib: bool) -> std::io::Result<()> {
    let main_file_path: String;
    match is_lib {
        true => main_file_path = format!("{project_path}/src/lib.rs"),
        false => main_file_path = format!("{project_path}/src/main.rs"),
    }
    
    let mut main_rs = OpenOptions::new()
        .write(true)
        .append(true)
        .open(main_file_path)?;
    
    main_rs.set_len(0)?;
    main_rs.write_all(new_content.as_bytes())?;
    
    Ok(())
}

/// Appends the default '\[lib\] proc-macro' to the end of the projects Cargo.toml file.
pub fn adjust_proc_macro(project_path: &str) -> std::io::Result<()> {
    let mut cargo_toml = OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open(format!("{project_path}/Cargo.toml"))?;
    
    let mut file_content_buffer = String::new();
    cargo_toml.read_to_string(&mut file_content_buffer)?;
    
    let deps_index = file_content_buffer.find("[dependencies]");
    match deps_index {
        Some(i) => {
            file_content_buffer.insert_str(i, PROC_MACRO_CARGO_TOML);
            
            cargo_toml.set_len(0)?;
            cargo_toml.write_all(file_content_buffer.as_bytes())?;
            
            Ok(())
        },
        None => {
            Err(Error::new(ErrorKind::NotFound, "Failed to read 'Cargo.toml'"))
        },
    }
}


const PROC_MACRO_CARGO_TOML: &str = "[lib]\nproc-macro = true\n\n";
