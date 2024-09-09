use crate::dataset::Dataset;
use crate::datasettype::DatasetType;
use crate::rawproperty::RawProperty;
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

        let mut zpool = Self {
            datasets: IndexMap::new(),
        };

        let raw_properties: Vec<RawProperty> = RawProperty::from_raw(&raw);

        for raw_property in raw_properties {

            let name: &String = &raw_property.dataset;
            let item = zpool.datasets.get_mut(name);
            match item {
                None => {
                    let mut new_dataset = Dataset::new(name);
                    new_dataset.fill(&raw_property);
                    zpool.datasets.insert(name.clone(), new_dataset);
                },
                _ => {
                    item.unwrap().fill(&raw_property);
                }
            }

        }

        let mut snapshots: Vec<String> = Vec::new();
        for (name, _) in zpool.datasets.iter() {
            if name.contains("@") {
                snapshots.push(name.clone());
            }
        }
        for name in snapshots {
            let (parent, child) = name.split_once("@").unwrap();
            let mut snapshot = zpool.datasets.shift_remove(&name).unwrap();
            snapshot.name = child.to_string();
            zpool.datasets.get_mut(parent).unwrap().add_snapshot(snapshot);
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

    pub fn len(&self) -> usize {

        self.datasets.len()

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

        for (_, dataset) in self.datasets.iter() {
            self.table_add_row(
                &mut table,
                dataset.name.clone(),
                dataset.used.value,
                dataset.referenced.value,
                dataset.compressratio.value,
                &dataset.datasettype.value,
            );
            for snapshot in &dataset.children {
                self.table_add_row(
                    &mut table,
                    snapshot.name.clone(),
                    snapshot.used.value,
                    snapshot.referenced.value,
                    snapshot.compressratio.value,
                    &snapshot.datasettype.value,
                );
            }
        }

        table.print();

    }

    fn table_add_row(
        &self,
        table: &mut Table,
        name: String,
        used: Option<u64>,
        referenced: Option<u64>,
        compressratio: Option<f32>,
        datasettype: &Option<DatasetType>,
    ) {

        let mut name = name.to_string();

        if datasettype == &Some(DatasetType::Snapshot) {
            name = name.bright_black().to_string();
        }

        let used = used.unwrap();
        let referenced = referenced.unwrap();
        let compressratio = compressratio.unwrap();

        let used_msg = used.to_string(); // TODO
        let referenced_msg = referenced.to_string(); // TODO

        let compressratio_msg = format!("{:.02}", compressratio);

        table.add_row(vec![name, used_msg, referenced_msg, compressratio_msg])

    }

}
