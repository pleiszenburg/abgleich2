use crate::dataset::Dataset;
use crate::rawproperty::RawProperty;

use std::collections::HashMap;
use std::io::Read;
use std::process::{Child, Command, Stdio};

pub struct Zpool {

    pub datasets: HashMap<String, Dataset>,

}

impl Zpool {

    pub fn from_raw(raw: String) -> Self {

        let mut zpool = Self {
            datasets: HashMap::new(),
        };

        let raw_properties: Vec<RawProperty> = RawProperty::from_raw(&raw);

        for raw_property in raw_properties {

            let name: String = raw_property.dataset.to_string();
            let item = zpool.datasets.get_mut(&name);
            match item {
                None => {
                    let mut new_dataset = Dataset::new(name.to_string());
                    new_dataset.fill(&raw_property);
                    zpool.datasets.insert(name.to_string(), new_dataset);
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
            let mut snapshot = zpool.datasets.remove(&name).unwrap();
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

        for (_, dataset) in self.datasets.iter() {
            println!("{:?} | {:?}", dataset.name, dataset.datasettype);
        }

    }

}
