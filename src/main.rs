use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() {

    let mut child = Command::new("zfs")
        .arg("get")
        .arg("all")
        .arg("-rHp")
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let mut child_stdout = child.stdout.as_mut().unwrap();

    let mut read_buffer = String::new();

    let read_res = child_stdout.read_to_string(&mut read_buffer);

    // // Close stdin to finish and avoid indefinite blocking
    // drop(child_stdout);

    let output = child.wait_with_output();

    println!("Yay!");
    println!("output = {:?}", output);
    println!("length = {:?}", read_buffer.bytes());

}
