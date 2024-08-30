use std::io::{Read};
use std::process::{Command, Stdio};

struct RawProperty {
    name: String,
    property: String,
    value: String,
}

/*

enum Origin {
    Inherited(String),
    Local,
    Stat,
}

enum Property {

}

*/

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

fn main() {

    let raw_output = cmd_zfs_all_rhp();

    let raw_properties: Vec<RawProperty> = lines_to_raw_properties(&raw_output);
    println!("len(line) == {:?}", raw_properties.len());

    println!("Yay!");

}
