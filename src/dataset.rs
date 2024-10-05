use crate::cmd::Cmd;
use crate::datasettype::DatasetType;
use crate::meta::Meta;
use crate::snapshot::Snapshot;
use crate::transaction::{Transaction, TransactionMeta};

pub struct Dataset {

    pub host: String,
    pub relname: Option<String>,
    pub meta: Meta,
    pub snapshots: Vec<Snapshot>,

}

impl Dataset {

    pub fn new(host: &str, root: &str, meta: Meta) -> Self {

        let relname: Option<String>;
        if meta.name == root {
            relname = None;
        } else {
            relname = Some(meta.name[root.len()+1..].to_string());
        }

        Self {
            host: host.to_string(),
            relname: relname,
            meta: meta,
            snapshots: Vec::new(),
        }

    }

    pub fn add_snapshot(&mut self, snapshot: Snapshot) {

        self.snapshots.push(snapshot);

    }

    pub fn contains_changes(&self, always_changed: bool, written_threshold: Option<u64>, check_diff: bool) -> bool {

        if always_changed {
            return true;
        }
        if self.snapshots.len() == 0 {
            return true;
        }
        if self.meta.written.value.unwrap() == 0 {
            return false;
        }
        if *self.meta.datasettype.value.as_ref().unwrap() == DatasetType::Volume {
            return true;
        }
        match written_threshold {
            Some(value) => {
                if value > self.meta.written.value.unwrap() {
                    return true;
                }
            },
            _ => {},
        }
        if !check_diff {
            return true;
        }

        let (raw, _) = Cmd::new(vec![
            "zfs".to_string(),
            "diff".to_string(),
            format!("{}@{}", self.meta.name, self.snapshots.last().unwrap().meta.name).to_string(),
        ]).on_host(&self.host, None).run();

        raw.trim_matches(&[' ', '\n', '\t']).len() > 0

    }

    pub fn get_snapshot_transaction(&self, snapshot_name: String) -> Transaction {

        Transaction::new(
            TransactionMeta::new_snapshot(
                self.meta.written.value.unwrap(),
                self.meta.name.clone(),
                snapshot_name.clone(),
            ),
            Cmd::new(vec![
                "zfs".to_string(),
                "snapshot".to_string(),
                format!("{}@{}", self.meta.name, snapshot_name),
            ]).on_host(&self.host, None),
        )

    }

}
