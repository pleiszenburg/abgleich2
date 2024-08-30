use std::io::{Read};
use std::process::{Command, Stdio};

struct RawProperty {
    name: String,
    property: String,
    value: String,
}

enum Origin {
    Inherited(String),
    Local,
    Default,
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

struct Stat<T> {
    value: Option<T>,
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

fn line_to_raw_property(line: &str) -> RawProperty{

    let mut fragments = line.split("\t");

    let raw_property = RawProperty {
        name: fragments.next().unwrap().to_string(),
        property: fragments.next().unwrap().to_string(),
        value: fragments.next().unwrap().to_string(),
    };

    raw_property

}

fn lines_to_raw_properties(raw: &String) -> Vec<RawProperty> {

    let lines = raw.split("\n");
    let chars: &[_] = &[' ', '\t'];
    let mut raw_properties: Vec<RawProperty> = Vec::new();

    for line in lines {
        let line_cleaned = line.trim_matches(chars);
        if line_cleaned.len() == 0 {
            continue;
        }
        let raw_property = line_to_raw_property(&line_cleaned);
        raw_properties.push(raw_property);
    }

    raw_properties

}

fn raw_properties_to_datasets(raw_properties: Vec<RawProperty>) -> Vec<Dataset> {

    let datasets = Vec::new();


    datasets

}

fn main() {

    let raw_output = cmd_zfs_all_rhp();

    let raw_properties: Vec<RawProperty> = lines_to_raw_properties(&raw_output);
    println!("len(raw_properties) == {:?}", raw_properties.len());

    let datasets = raw_properties_to_datasets(raw_properties);
    println!("len(datasets) == {:?}", datasets.len());

    println!("Yay!");

}
