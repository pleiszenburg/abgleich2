#[derive(PartialEq)]
#[derive(Debug)]
pub enum DatasetType {
    Filesystem,
    Volume,
    Snapshot,
}

impl DatasetType {
    pub fn from_raw(raw: &str) -> Self {
        match raw {
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
