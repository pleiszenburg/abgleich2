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

    pub fn run(&self) -> (String, String) {

        let mut command1 = Command::new(
            OsStr::new(self.fragments[0].as_str())
        );
        let command2 = command1.args(
            self.fragments[1..].iter()
        ).stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = command2.spawn().expect("command spawn into child failed");

        let child_stdout = child.stdout.as_mut().expect("failed to attach to child stdout");
        let child_stderr = child.stderr.as_mut().expect("failed to attach to child stderr");

        let mut stdout_buffer = String::new();
        let mut stderr_buffer = String::new();

        child_stdout.read_to_string(&mut stdout_buffer).expect("failed to read from child stdout");
        child_stderr.read_to_string(&mut stderr_buffer).expect("failed to read from child stderr");

        child.wait().expect("child finished with errors");

        (stdout_buffer, stderr_buffer)

    }

    pub fn to_string(&self) -> String {

        self.fragments.join(" ")

    }

}
