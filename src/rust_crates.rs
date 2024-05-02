use std::fs::File;
use std::io::Write;


#[allow(unused)]
pub enum RustCrates {
    Syn,
    Quote,
    Libc,
    Rand,
    Serde,
    SerdeJson,
    Bytes,
    Rocket,
    Tungstenite,
}
impl RustCrates {
    pub fn get_import_string(&self) -> &str {
        match self {
            Self::Syn => "syn = \"2.0\"",
            Self::Quote => "quote = \"1.0\"",
            Self::Libc => "libc = \"0.2\"",
            Self::Rand => "rand = \"0.8.4\"",
            Self::Serde => "serde = { version = \"1.0\", features = [\"derive\"] }",
            Self::SerdeJson => "serde_json = \"1.0\"",
            Self::Bytes => "bytes = \"1\"",
            Self::Rocket => "rocket = \"0.5.0\"",
            Self::Tungstenite => "tungstenite = \"0.21.0\"",
        }
    }
    
    /// Appends the packages import string to the provided File, usually './project-name/Cargo.toml'.
    pub fn append_import_to_file(&self, file: &mut File) {
        let import_string = self.get_import_string();
        file.write_all(import_string.as_bytes())
            .expect("Failed to write import string to Cargo.toml.");
    }
}
