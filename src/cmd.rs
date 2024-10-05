use std::ffi::OsStr;
use std::io::Read;
use std::process::{Command, Stdio};

use shlex::{try_join, split};

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

    pub fn on_host(&self, host: &str, args: Option<&str>) -> Self {

        if host == "localhost" {
            return self.clone();
        }

        let mut fragments: Vec<String> = vec!["ssh".to_string()];

        match args {
            Some(args) => {
                let mut args_fragments = split(args).expect("could not split args");
                fragments.append(&mut args_fragments);
            }
            _ => {}
        }

        fragments.push(host.to_string());

        let mut self_fragments_ref: Vec<&str> = Vec::new();
        for fragment in self.fragments.iter() {
            self_fragments_ref.push(&fragment.as_str());
        }

        fragments.push(try_join(self_fragments_ref).expect("could not join command"));

        Self::new(fragments)

    }

}

impl Clone for Cmd {

    fn clone(&self) -> Self {

        let mut new_fragments: Vec<String> = Vec::new();
        for fragment in &self.fragments {
            new_fragments.push(fragment.clone());
        }

        Self::new(new_fragments)

    }

}
