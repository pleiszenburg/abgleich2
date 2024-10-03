use crate::meta::Meta;

pub struct Snapshot {

    pub parent: String,
    pub meta: Meta,

}

impl Snapshot {

    pub fn new (parent: String, meta: Meta) -> Self {
        Self {
            parent: parent,
            meta: meta,
        }
    }

}
