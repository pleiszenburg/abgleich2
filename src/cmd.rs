use std::ffi::OsStr;
use std::io::Read;
use std::process::{Command, Stdio};

pub struct Cmd {

    fragments: Vec<String>,

}

impl Cmd {

    pub fn new(fragments: Vec<String>) -> Self {
        assert!(fragments.len() > 0);
        Self {
            fragments: fragments,
        }
    }

    pub fn run(&self) -> String {

        let mut command1 = Command::new(
            OsStr::new(self.fragments[0].as_str())
        );
        let command2 = command1.args(
            self.fragments[1..].iter()
        ).stdout(Stdio::piped());

        match command2.spawn() {
            Ok(mut child) => {
                match child.stdout.as_mut() {
                    Some(child_stdout) => {
                        let mut stdout_buffer = String::new();
                        match child_stdout.read_to_string(&mut stdout_buffer) {
                            Ok(_) => {
                                match child.wait() {
                                    Ok(_) => {
                                        stdout_buffer
                                    },
                                    Err(err) => {
                                        panic!("command finished with errors\n{}", err);
                                    }
                                }
                            },
                            Err(err) => {
                                panic!("failed to read from command stdout\n{}", err);
                            }
                        }
                    },
                    _ => {
                        panic!("failed to attach to command stdout");
                    },
                }
            },
            Err(err) => {
                panic!("command spawn failed\n{}", err);
            }
        }

    }

}
