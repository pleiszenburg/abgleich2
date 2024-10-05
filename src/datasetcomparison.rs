use crate::dataset::Dataset;

use std::collections::HashSet;

pub struct DatasetComparison<'a> {
    source: Option<&'a Dataset>,
    target: Option<&'a Dataset>,
}

impl DatasetComparison<'_> {

    pub fn new<'b>(source: Option<&'b Dataset>, target: Option<&'b Dataset>) -> DatasetComparison<'b> {

        DatasetComparison {
            source: source,
            target: target,
        }

    }

    pub fn get_redundant_snapshots(&self) -> Vec<String> {

        match (self.source, self.target) {
            (Some(source), Some(target)) => {

                let mut source_snapshots: HashSet<String> = HashSet::new();
                for source_snapshot in source.snapshots.iter() {
                    source_snapshots.insert(source_snapshot.meta.name.clone());
                }

                let mut redundant_snapshots: Vec<String> = Vec::new();
                for target_snapshot in target.snapshots.iter() {
                    if source_snapshots.contains(&target_snapshot.meta.name) {
                        redundant_snapshots.push(target_snapshot.meta.name.clone());
                    }
                }

                if redundant_snapshots.len() < 2 {
                    return redundant_snapshots;
                }

                let mut idx = 0;
                let mut found: bool;
                for redundant_snapshot in redundant_snapshots.iter() {
                    found = false;
                    while (idx < source.snapshots.len()) & !found {
                        found = source.snapshots[idx].meta.name == *redundant_snapshot;
                        idx += 1;
                    }
                    if (idx >= source.snapshots.len()) & !found {
                        panic!("redundant snapshot sequence validation failed on source");
                    }
                }

                redundant_snapshots

            },
            _ => {

                vec![]

            },
        }

    }

    pub fn print_table(&self) {

        let source_status = self.dataset_status(self.source);
        let target_status = self.dataset_status(self.target);

        println!("{} <=> {}", source_status, target_status);

        let redundant_snapshots = self.get_redundant_snapshots();
        for snapshot in redundant_snapshots.iter() {
            println!("  - {}", snapshot);
        }

    }

    fn dataset_status(&self, dataset: Option<&Dataset>) -> String {

        match dataset {
            Some(dataset) => {
                format!("{}:{}", dataset.host, dataset.meta.name)
            },
            _ => {
                "[none]".to_string()
            }
        }

    }

}
