use std::io::{Read};
use std::process::{Command, Stdio};

fn main() {

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

    println!("Yay!");

}
