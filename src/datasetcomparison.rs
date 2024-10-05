use crate::dataset::Dataset;

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

    pub fn print_table(&self) {

        let source_status = self.dataset_status(self.source);
        let target_status = self.dataset_status(self.target);
        println!("{} <=> {}", source_status, target_status);

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
