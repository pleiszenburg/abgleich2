use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio};

mod rawproperty;
use rawproperty::RawProperty;

enum Origin {
    Inherited(String),
    Local,
    Default,
}

impl Origin {
    fn from_raw(raw: String) -> Self {
        if raw == "local".to_string() {
            return Self::Local;
        }
        if raw == "default".to_string() {
            return Self::Default;
        }
        if raw.starts_with("inherited from") {
            return Self::Inherited(raw[14..].to_string());
        }
        panic!("expected origin");
    }
}

enum DatasetType {
    Filesystem,
    Volume,
    Snapshot,
}

impl DatasetType {
    fn from_raw(raw: String) -> Self {
        match raw.as_str() {
            "filesystem" => {
                Self::Filesystem
            }
            "volume" => {
                Self::Volume
            }
            "snapshot" => {
                Self::Snapshot
            }
            _ => {
                panic!("expected dataset type");
            }
        }
    }
}

struct Property<T> {
    value: Option<T>,
    origin: Option<Origin>,
}

impl<T> Property<T> {
    fn from_empty() -> Property<T> {
        Property {
            value: None,
            origin: None,
        }
    }
}

impl Property<DatasetType> {
    fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(DatasetType::from_raw(raw_property.value.clone()));
        self.origin = Some(Origin::from_raw(raw_property.meta.clone()));
    }
}

impl Property<bool> {
    fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(parse_onoff(raw_property.value.clone()));
        self.origin = Some(Origin::from_raw(raw_property.meta.clone()));
    }
}

impl Property<i64> {
    fn fill(&mut self, raw_property: &RawProperty) {
        let result = raw_property.value.parse::<i64>();
        match result {
            Ok(number) => {
                self.value = Some(number);
            }
            Err(error) => {
                panic!("i64 parser fail on {:?} with {:?}", raw_property, error);
            }
        }
        self.origin = Some(Origin::from_raw(raw_property.meta.clone()));
    }
}

impl Property<u64> {
    fn fill(&mut self, raw_property: &RawProperty) {
        let result = raw_property.value.parse::<u64>();
        match result {
            Ok(number) => {
                self.value = Some(number);
            }
            Err(error) => {
                panic!("u64 parser fail on {:?} with {:?}", raw_property, error);
            }
        }
        self.origin = Some(Origin::from_raw(raw_property.meta.clone()));
    }
}

impl Property<String> {
    fn fill(&mut self, raw_property: &RawProperty) {
        self.value = Some(raw_property.value.clone());
        self.origin = Some(Origin::from_raw(raw_property.meta.clone()));
    }
}

struct Stat<T> {
    value: Option<T>,
}

impl<T> Stat<T> {
    fn from_empty () -> Stat<T> {
        Stat{
            value: None,
        }
    }
}

struct Dataset {

    name: String,

    atime: Property<bool>,
    canmount: Property<bool>,
    checksum: Property<bool>,
    compression: Property<String>,
    datasettype: Property<DatasetType>,
    dedup: Property<bool>,
    encryption: Property<bool>,
    filesystem_count: Property<u64>,
    filesystem_limit: Property<u64>,
    mountpoint: Property<String>,
    readonly: Property<bool>,
    redundant_metadata: Property<String>,
    relatime: Property<bool>,
    sharenfs: Property<bool>,
    snapshot_count: Property<u64>,
    snapshot_limit: Property<u64>,
    sync: Property<String>,
    volmode: Property<String>,

    available: Stat<i64>,
    compressratio: Stat<f32>,
    creation: Stat<i64>,
    guid: Stat<i64>,
    logicalreferenced: Stat<i64>,
    logicalused: Stat<i64>,
    mounted: Stat<bool>,
    refcompressratio: Stat<f32>,
    referenced: Stat<i64>,
    used: Stat<i64>,
    usedbychildren: Stat<i64>,
    usedbydataset: Stat<i64>,
    usedbyrefreservation: Stat<i64>,
    usedbysnapshots: Stat<i64>,
    version: Stat<i64>,
    written: Stat<i64>,

}

impl Dataset {

    fn new (name: String) -> Self {

        Self {

            name: name,

            atime: Property::from_empty(),
            canmount: Property::from_empty(),
            checksum: Property::from_empty(),
            compression: Property::from_empty(),
            datasettype: Property::from_empty(),
            dedup: Property::from_empty(),
            encryption: Property::from_empty(),
            filesystem_count: Property::from_empty(),
            filesystem_limit: Property::from_empty(),
            mountpoint: Property::from_empty(),
            readonly: Property::from_empty(),
            redundant_metadata: Property::from_empty(),
            relatime: Property::from_empty(),
            sharenfs: Property::from_empty(),
            snapshot_count: Property::from_empty(),
            snapshot_limit: Property::from_empty(),
            sync: Property::from_empty(),
            volmode: Property::from_empty(),

            available: Stat::from_empty(),
            compressratio: Stat::from_empty(),
            creation: Stat::from_empty(),
            guid: Stat::from_empty(),
            logicalreferenced: Stat::from_empty(),
            logicalused: Stat::from_empty(),
            mounted: Stat::from_empty(),
            refcompressratio: Stat::from_empty(),
            referenced: Stat::from_empty(),
            used: Stat::from_empty(),
            usedbychildren: Stat::from_empty(),
            usedbydataset: Stat::from_empty(),
            usedbyrefreservation: Stat::from_empty(),
            usedbysnapshots: Stat::from_empty(),
            version: Stat::from_empty(),
            written: Stat::from_empty(),

        }

    }

}

fn cmd_zfs_all_rhp() -> String {

    let mut proc = Command::new("zfs")
        .arg("get")
        .arg("all")
        .arg("-rHp")
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let proc_stdout = proc.stdout.as_mut().unwrap();
    let mut stdout_buffer = String::new();

    let _read_res = proc_stdout.read_to_string(&mut stdout_buffer);

    let _output = proc.wait_with_output();

    stdout_buffer

}

fn parse_onoff(raw: String) -> bool {
    match raw.as_str() {
        "on" => { true }
        "off" => { false }
        _ => { panic!("expected on/off bool") }
    }
}

fn raw_property_to_value(dataset: &mut Dataset, raw_property: &RawProperty) {

    match raw_property.name.as_str() {
        "atime" => { dataset.atime.fill(raw_property) },
        "canmount" => { dataset.canmount.fill(raw_property) },
        "checksum" => { dataset.checksum.fill(raw_property) },
        "compression" => { dataset.compression.fill(raw_property) },
        "datasettype" => { dataset.datasettype.fill(raw_property) },
        "encryption" => { dataset.encryption.fill(raw_property) },
        "filesystem_count" => { dataset.filesystem_count.fill(raw_property) },
        "filesystem_limit" => { dataset.filesystem_limit.fill(raw_property) },
        "mountpoint" => { dataset.mountpoint.fill(raw_property) },
        "readonly" => { dataset.readonly.fill(raw_property) },
        "redundant_metadata" => { dataset.redundant_metadata.fill(raw_property) },
        "relatime" => { dataset.relatime.fill(raw_property) },
        "sharenfs" => { dataset.sharenfs.fill(raw_property) },
        "snapshot_count" => { dataset.snapshot_count.fill(raw_property) },
        "snapshot_limit" => { dataset.snapshot_limit.fill(raw_property) },
        "sync" => { dataset.sync.fill(raw_property) },
        "volmode" => { dataset.volmode.fill(raw_property) },
        _ => { /* unknown parameter */ }

        /*

        available
        compressratio
        creation
        guid
        logicalreferenced
        logicalused
        mounted
        refcompressratio
        referenced
        used
        usedbychildren
        usedbydataset
        usedbyrefreservation
        usedbysnapshots
        version
        written

         */
    }

}

fn raw_properties_to_datasets(raw_properties: Vec<RawProperty>) -> HashMap<String, Dataset> {

    let mut datasets: HashMap<String, Dataset> = HashMap::new();

    for raw_property in raw_properties {

        let name: String = raw_property.dataset.to_string();
        let item = datasets.get_mut(&name);
        match item {
            None => {
                let mut new_dataset = Dataset::new(name.to_string());
                raw_property_to_value(&mut new_dataset, &raw_property);
                datasets.insert(name.to_string(), new_dataset);
            },
            _ => {
                raw_property_to_value(&mut item.unwrap(), &raw_property);
            }
        }

    }

    datasets

}

fn main() {

    let raw_output = cmd_zfs_all_rhp();

    let raw_properties: Vec<RawProperty> = RawProperty::from_raw(&raw_output);
    println!("len(raw_properties) == {:?}", raw_properties.len());

    let datasets = raw_properties_to_datasets(raw_properties);
    println!("len(datasets) == {:?}", datasets.len());

    println!("Yay!");

}
