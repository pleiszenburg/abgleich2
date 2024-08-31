use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio};

struct RawProperty {
    dataset: String,
    name: String,
    value: String,
    meta: String,
}

impl RawProperty {

    fn from_line(line: &str) -> Self{

        let mut fragments = line.split("\t");

        let raw_property = Self {
            dataset: fragments.next().unwrap().to_string(),
            name: fragments.next().unwrap().to_string(),
            value: fragments.next().unwrap().to_string(),
            meta: fragments.next().unwrap().to_string(),
        };

        raw_property

    }

    fn from_raw(raw: &String) -> Vec<Self> {

        let lines = raw.split("\n");
        let chars: &[_] = &[' ', '\t'];
        let mut raw_properties: Vec<Self> = Vec::new();

        for line in lines {
            let line_cleaned = line.trim_matches(chars);
            if line_cleaned.len() == 0 {
                continue;
            }
            let raw_property = Self::from_line(&line_cleaned);
            raw_properties.push(raw_property);
        }

        raw_properties

    }

}

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

struct Property<T> {
    value: Option<T>,
    origin: Option<Origin>,
}

impl<T> Property<T> {
    fn empty () -> Property<T> {
        Property{
            value: None,
            origin: None,
        }
    }
}

struct Stat<T> {
    value: Option<T>,
}

impl<T> Stat<T> {
    fn empty () -> Stat<T> {
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
    filesystem_count: Property<i64>,
    filesystem_limit: Property<i64>,
    mountpoint: Property<String>,
    readonly: Property<bool>,
    redundant_metadata: Property<String>,
    relatime: Property<bool>,
    sharenfs: Property<bool>,
    snapshot_count: Property<i64>,
    snapshot_limit: Property<i64>,
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
    fn new (name: String) -> Self{
        Self {

            name: name,

            atime: Property::empty(),
            canmount: Property::empty(),
            checksum: Property::empty(),
            compression: Property::empty(),
            datasettype: Property::empty(),
            dedup: Property::empty(),
            encryption: Property::empty(),
            filesystem_count: Property::empty(),
            filesystem_limit: Property::empty(),
            mountpoint: Property::empty(),
            readonly: Property::empty(),
            redundant_metadata: Property::empty(),
            relatime: Property::empty(),
            sharenfs: Property::empty(),
            snapshot_count: Property::empty(),
            snapshot_limit: Property::empty(),
            sync: Property::empty(),
            volmode: Property::empty(),

            available: Stat::empty(),
            compressratio: Stat::empty(),
            creation: Stat::empty(),
            guid: Stat::empty(),
            logicalreferenced: Stat::empty(),
            logicalused: Stat::empty(),
            mounted: Stat::empty(),
            refcompressratio: Stat::empty(),
            referenced: Stat::empty(),
            used: Stat::empty(),
            usedbychildren: Stat::empty(),
            usedbydataset: Stat::empty(),
            usedbyrefreservation: Stat::empty(),
            usedbysnapshots: Stat::empty(),
            version: Stat::empty(),
            written: Stat::empty(),

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
        "atime" => {
            dataset.atime.value = Some(parse_onoff(raw_property.value.clone()));
            dataset.atime.origin = Some(Origin::from_raw(raw_property.meta.clone()));
        }
        _ => {
            // unknown parameter
        }
        /*

        atime,
        canmount
        checksum:
        compression
        datasettype
        dedup
        encryption
        filesystem_count
        filesystem_limit
        mountpoint
        readonly
        redundant_metadata
        relatime
        sharenfs
        snapshot_count
        snapshot_limit
        sync
        volmode

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
