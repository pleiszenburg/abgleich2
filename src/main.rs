use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() {

    let mut child = Command::new("zfs get all -rHp")
        .stdout(Stdio::piped())
        .spawn();

    let child_stdout = child.stdout.as_mut(); // .unwrap();

    let read_buffer = String::new();

    let read_res = child_stdout.read_to_string(&mut read_buffer);

    // Close stdin to finish and avoid indefinite blocking
    drop(child_stdout);

    let output = child.wait_with_output();

    println!("output = {:?}", output);
    println!("length = {:?}", read_buffer.bytes())

    Ok(())

}
