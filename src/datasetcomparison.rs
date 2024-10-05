use crate::dataset::Dataset;

pub struct DatasetComparison {
    source: Option<Dataset>,
    target: Option<Dataset>,
}

impl DatasetComparison {

    pub fn new(source: Option<Dataset>, target: Option<Dataset>) -> Self {
        Self {
            source: source,
            target: target,
        }
    }

    pub fn print_table(&self) {
        let source_status = self.dataset_status(&self.source);
        let target_status = self.dataset_status(&self.target);
        println!("{} | {}", source_status, target_status);
    }

    fn dataset_status(&self, dataset: &Option<Dataset>) -> String {
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
