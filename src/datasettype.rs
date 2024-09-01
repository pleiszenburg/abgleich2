pub enum DatasetType {
    Filesystem,
    Volume,
    Snapshot,
}

impl DatasetType {
    pub fn from_raw(raw: String) -> Self {
        match raw.as_str() {
            "filesystem" => {
                Self::Filesystem
            }
            "volume" => {
                Self::Volume
            }
            "snapshot" => {
                Self::Snapshot
            }
            _ => {
                panic!("expected dataset type");
            }
        }
    }
}
