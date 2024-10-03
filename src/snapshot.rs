use crate::meta::Meta;

pub struct Snapshot {

    pub meta: Meta,

}

impl Snapshot {

    pub fn new (meta: Meta) -> Self {
        Self {
            meta: meta,
        }
    }

}
