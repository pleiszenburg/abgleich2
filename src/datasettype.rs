#[derive(PartialEq, Debug)]
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

    // pub fn to_string(&self) -> String {
    //     match &self {
    //         Self::Filesystem => {
    //             "filesystem".to_string()
    //         }
    //         Self::Volume => {
    //             "volume".to_string()
    //         }
    //         Self::Snapshot => {
    //             "snapshot".to_string()
    //         }
    //         _ => {
    //             panic!("expected dataset type");
    //         }
    //     }
    // }

}
