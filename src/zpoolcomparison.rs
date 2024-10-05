use crate::datasetcomparison::DatasetComparison;
use crate::zpool::Zpool;

use std::collections::HashSet;

pub struct ZpoolComparison {
    source: Zpool,
    target: Zpool,
}

impl ZpoolComparison {

    pub fn new(source: Zpool, target: Zpool) -> Self {

        Self {
            source: source,
            target: target,
        }

    }

    fn get_unique_datasets(&self) -> Vec<&Option<String>> {

        let mut unique_dataset_set: HashSet<&Option<String>> = HashSet::new();
        for dataset in self.source.datasets.values() {
            unique_dataset_set.insert(&dataset.relname);
        }
        for dataset in self.target.datasets.values() {
            unique_dataset_set.insert(&dataset.relname);
        }
        let mut unique_datasets: Vec<&Option<String>> = Vec::new();
        for dataset in unique_dataset_set.drain() {
            unique_datasets.push(dataset);
        }
        drop(unique_dataset_set);
        unique_datasets.sort();

        unique_datasets

    }

    fn get_dataset_comparisons(&self, unique_datasets: Vec<&Option<String>>) -> Vec<DatasetComparison> {

        let mut dataset_comparisons: Vec<DatasetComparison> = Vec::new();

        for relname in unique_datasets {
            dataset_comparisons.push(DatasetComparison::new(
                self.source.get_dataset_by_relname(relname),
                self.target.get_dataset_by_relname(relname),
            ));
        }

        dataset_comparisons

    }

    pub fn print_table(&self) {

        println!("Compare: {} vs {}", self.source.len(), self.target.len());

        let unique_datasets = self.get_unique_datasets();
        let dataset_comparisons: Vec<DatasetComparison> = self.get_dataset_comparisons(unique_datasets);

        for dataset_comparison in dataset_comparisons {
            dataset_comparison.print_table();
        }

    }

}
