#[derive(Debug)]
pub enum Origin {
    Inherited(String),
    Local,
    Default,
}

impl Origin {
    pub fn from_raw(raw: &str) -> Self {
        if raw == "local" {
            return Self::Local;
        }
        if raw == "default" {
            return Self::Default;
        }
        if raw.starts_with("inherited from") {
            return Self::Inherited(raw[14..].to_string());
        }
        panic!("expected origin {:?}", raw);
    }
}
