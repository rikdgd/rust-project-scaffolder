pub enum PopularCrates {
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
impl PopularCrates {
    pub fn get_import_string(&self) -> String {
        match self {
            Self::Syn => "syn = \"2.0\"".to_string(),
            Self::Quote => "quote = \"1.0\"".to_string(),
            Self::Libc => "libc = \"0.2\"".to_string(),
            Self::Rand => "rand = \"0.8.4\"".to_string(),
            Self::Serde => "serde = { version = \"1.0\", features = [\"derive\"] }".to_string(),
            Self::SerdeJson => "serde_json = \"1.0\"".to_string(),
            Self::Bytes => "bytes = \"1\"".to_string(),
            Self::Rocket => "rocket = \"0.5.0\"".to_string(),
            Self::Tungstenite => "tungstenite = \"0.21.0\"".to_string(),
        }
    }
}
