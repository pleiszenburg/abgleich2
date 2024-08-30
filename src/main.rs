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

fn zfs_all() -> String {

    let mut child = Command::new("zfs")
        .arg("get")
        .arg("all")
        .arg("-rHp")
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let child_stdout = child.stdout.as_mut().unwrap();

    let mut read_buffer = String::new();
    let _read_res = child_stdout.read_to_string(&mut read_buffer);
    // println!("length = {:?}", read_buffer.bytes());

    let _output = child.wait_with_output();
    // println!("output = {:?}", output);

    read_buffer

}

fn parse_line(line: &str) -> RawProperty{

    let mut fragments = line.split("\t");

    let field = RawProperty {
        name: fragments.next().unwrap().to_string(),
        property: fragments.next().unwrap().to_string(),
        value: fragments.next().unwrap().to_string(),
    };

    field

}

fn parse_lines(raw: &String) -> i64 {

    let lines = raw.split("\n");
    let chars: &[_] = &[' ', '\t'];

    let mut count: i64 = 0;
    for line in lines {
        let line_cleaned = line.trim_matches(chars);
        if line_cleaned.len() == 0 {
            continue;
        }
        let _field = parse_line(&line_cleaned);
        count += 1;
    }

    count

}

fn main() {

    let raw = zfs_all();

    let count = parse_lines(&raw);
    println!("len(line) == {:?}", count);

    println!("Yay!");

}
