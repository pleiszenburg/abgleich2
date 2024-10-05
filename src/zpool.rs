use crate::cmd::Cmd;
use crate::dataset::Dataset;
use crate::datasettype::DatasetType;
use crate::meta::Meta;
use crate::misc::colorized_storage_si_suffix;
use crate::rawproperty::RawProperty;
use crate::snapshot::Snapshot;
use crate::table::{Table, Alignment};
use crate::transaction::TransactionList;

use chrono::{Utc, DateTime};
use colored::Colorize;
use indexmap::IndexMap;

pub struct Zpool {

    host: String,
    root: String,
    datasets: IndexMap<String, Dataset>,

}

impl Zpool {

    pub fn from_raw(host: &str, root: &str, raw: String) -> Self {

        let raw_properties: Vec<RawProperty> = RawProperty::from_raw(&raw);

        let mut metas: IndexMap<String, Meta> = IndexMap::new();
        for raw_property in raw_properties {
            let name: &String = &raw_property.dataset;
            let item = metas.get_mut(name);
            match item {
                None => {
                    let mut new_dataset = Meta::new(name);
                    new_dataset.fill(&raw_property);
                    metas.insert(name.clone(), new_dataset);
                },
                Some(value) => {
                    value.fill(&raw_property);
                }
            }
        }

        let mut datasets: Vec<String> = Vec::new();
        let mut snapshots: Vec<String> = Vec::new();
        for (name, meta) in metas.iter() {
            match meta.datasettype.value.as_ref().unwrap() {
                DatasetType::Snapshot => {
                    snapshots.push(name.clone());
                },
                _ => {
                    datasets.push(name.clone());
                }
            }
        }

        metas.reverse();  // a bit of performance later on, i.e. less shifting?
        let mut zpool = Self {
            host: host.to_string(),
            root: root.to_string(),
            datasets: IndexMap::new(),
        };
        for name in datasets {
            let meta = metas.shift_remove(&name).unwrap();
            zpool.datasets.insert(name.clone(), Dataset::new(host, meta));
        }
        for name in snapshots {
            let mut meta = metas.shift_remove(&name).unwrap();
            let (parent, child) = name.split_once("@").unwrap();
            meta.name = child.to_string();
            zpool.datasets.get_mut(parent).unwrap().add_snapshot(Snapshot::new(parent.to_string(), meta));
        }

        zpool

    }

    pub fn from_cmd(host: &str, root: &str) -> Self {

        let (raw, _) = Cmd::new(vec![
            "zfs".to_string(),
            "get".to_string(),
            "all".to_string(),
            "-rHp".to_string(),
            root.to_string(),
        ]).on_host(host, None).run();

        Self::from_raw(host, root, raw)

    }

    pub fn len(&self) -> usize {

        self.datasets.len()

    }

    pub fn get_snapshot_transaction(
        &self,
        always_changed: bool,
        written_threshold: Option<u64>,
        check_diff: bool,
        suffix: &str,
        ignore: &Vec<String>,
    ) -> TransactionList{

        let mut transactions = TransactionList::new();

        for (name, dataset) in self.datasets.iter() {
            if ignore.iter().any(|e| format!("{}/{}", self.root, e) == *name) {
                continue;
            }
            if !dataset.contains_changes(always_changed, written_threshold, check_diff) {
                continue;
            }
            transactions.append(dataset.get_snapshot_transaction(self.get_snapshot_name(suffix)));
        }

        transactions

    }

    fn get_snapshot_name(&self, suffix: &str) -> String {

        let now: DateTime<Utc> = Utc::now();

        format!("{}{}", now.format("%Y%m%d_%H%M%S"), suffix)

    }

    pub fn print_tree(&self) {

        let mut table = Table::from_head(
            vec![
                "name".to_string(),
                "used".to_string(),
                "referenced".to_string(),
                "compressratio".to_string(),
            ],
            vec![
                Alignment::Left,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
            ]
        );

        for dataset in self.datasets.values() {
            self.table_add_row(
                &mut table,
                &format!("{}:{}", self.host, dataset.meta.name),
                &dataset.meta.used.value,
                &dataset.meta.referenced.value,
                &dataset.meta.compressratio.value,
                &dataset.meta.datasettype.value,
            );
            for snapshot in &dataset.snapshots {
                self.table_add_row(
                    &mut table,
                    &format!("- {}", snapshot.meta.name),
                    &snapshot.meta.used.value,
                    &snapshot.meta.referenced.value,
                    &snapshot.meta.compressratio.value,
                    &snapshot.meta.datasettype.value,
                );
            }
        }

        table.print();

    }

    fn table_add_row(
        &self,
        table: &mut Table,
        name: &String,
        used: &Option<u64>,
        referenced: &Option<u64>,
        compressratio: &Option<f32>,
        datasettype: &Option<DatasetType>,
    ) {

        let mut name = name.to_string();

        if datasettype == &Some(DatasetType::Snapshot) {
            name = name.bright_black().to_string();
        }

        let used = used.unwrap();
        let referenced = referenced.unwrap();
        let compressratio = compressratio.unwrap();

        let used_msg = colorized_storage_si_suffix(used); // TODO
        let referenced_msg = colorized_storage_si_suffix(referenced); // TODO

        let compressratio_msg = format!("{:.02}", compressratio);

        table.add_row(&vec![name, used_msg, referenced_msg, compressratio_msg])

    }

}
