use std::{
    io::Write,
    process::{Command, Stdio},
};

pub struct ChildPipedProcess<'a> {
    name: &'a str,
    args: &'a [&'a str],
    piped_input: Option<&'a Vec<u8>>,
    output: Vec<u8>,
}

impl<'a> ChildPipedProcess<'a> {
    pub fn new(
        name: &'a str,
        args: &'a [&'a str],
        piped_input: Option<&'a Vec<u8>>,
    ) -> ChildPipedProcess<'a> {
        ChildPipedProcess {
            name,
            args,
            piped_input,
            output: vec![],
        }
    }

    pub fn process_output(&mut self) -> &Self {
        match self.piped_input {
            None => {
                let child = Command::new(self.name)
                    .args(self.args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("couldn't create the child");

                drop(self.args);

                let mut header_output = child.wait_with_output().expect("child-out").stdout;

                self.output.append(&mut header_output);
            }
            Some(data) => {
                for b in data.chunks(32000) {
                    let mut process = Command::new(self.name)
                        .args(self.args)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("couldn't create the child");

                    let child_stdin = process.stdin.as_mut().unwrap();

                    child_stdin
                        .write(b)
                        .expect("Something went wrong with writing chunks to the pipe");

                    let mut header_output = process.wait_with_output().expect("child-out").stdout;

                    self.output.append(&mut header_output);
                }
            }
        };

        self
    }

    pub fn output_to_str(&self) -> String {
        let out = String::from_utf8(self.output.to_vec())
            .expect("something went wrong with converting u8 vec to str");
        out
    }
}

