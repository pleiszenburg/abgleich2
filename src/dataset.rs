use crate::meta::Meta;
use crate::snapshot::Snapshot;

pub struct Dataset {

    pub meta: Meta,
    pub snapshots: Vec<Snapshot>,

}

impl Dataset {

    pub fn new(meta: Meta) -> Self {
        Self {
            meta: meta,
            snapshots: Vec::new(),
        }
    }

    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot);
    }

}
