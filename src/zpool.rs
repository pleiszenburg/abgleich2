use crate::dataset::Dataset;
use crate::datasettype::DatasetType;
use crate::meta::Meta;
use crate::misc::colorized_storage_si_suffix;
use crate::rawproperty::RawProperty;
use crate::snapshot::Snapshot;
use crate::table::{Table, Alignment};

use colored::Colorize;
use indexmap::IndexMap;

// use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio};

pub struct Zpool {

    datasets: IndexMap<String, Dataset>,

}

impl Zpool {

    pub fn from_raw(raw: String) -> Self {

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
            datasets: IndexMap::new(),
        };
        for name in datasets {
            let meta = metas.shift_remove(&name).unwrap();
            zpool.datasets.insert(name.clone(), Dataset::new(meta));
        }
        for name in snapshots {
            let mut meta = metas.shift_remove(&name).unwrap();
            let (parent, child) = name.split_once("@").unwrap();
            meta.name = child.to_string();
            zpool.datasets.get_mut(parent).unwrap().add_snapshot(Snapshot::new(parent.to_string(), meta));
        }

        zpool

    }

    pub fn from_local() -> Self {

        let mut proc = Command::new("zfs")
            .arg("get")
            .arg("all")
            .arg("-rHp")
            .stdout(Stdio::piped())
            .spawn().unwrap();

        let proc_stdout = proc.stdout.as_mut().unwrap();
        let mut stdout_buffer = String::new();

        let _read_res = proc_stdout.read_to_string(&mut stdout_buffer); // TODO

        let _output = proc.wait_with_output(); // TODO

        Self::from_raw(stdout_buffer)

    }

    // pub fn len(&self) -> usize {

    //     self.datasets.len()

    // }

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
                &dataset.meta.name,
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
