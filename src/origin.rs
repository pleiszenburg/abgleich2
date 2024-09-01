pub enum Origin {
    Inherited(String),
    Local,
    Default,
}

impl Origin {
    pub fn from_raw(raw: String) -> Self {
        if raw == "local".to_string() {
            return Self::Local;
        }
        if raw == "default".to_string() {
            return Self::Default;
        }
        if raw.starts_with("inherited from") {
            return Self::Inherited(raw[14..].to_string());
        }
        panic!("expected origin");
    }
}
