use crate::datasettype::DatasetType;
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

    pub fn is_changed(&self, always_changed: bool, written_threshold: Option<u64>, check_diff: bool) -> bool {

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

        // output, _ = (
        //     Command.from_list(
        //         ["zfs", "diff", f"{self._name:s}@{self._snapshots[-1].name:s}"]
        //     )
        //     .on_side(side=self._side, config=self._config)
        //     .run()
        // )
        // return len(output[0].strip(" \t\n")) > 0

        false  // TODO

    }

}
