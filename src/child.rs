use std::process::{Command, Stdio};

use tokio_file_unix::raw_stdin;

pub struct ChildPipedProcess<'a> {
    name: &'a str,
    args: &'a [&'a str],
    output: Vec<u8>,
}

impl <'a> ChildPipedProcess <'a>{


    pub fn new(name: &'a str, args: &'a [&'a str]) -> ChildPipedProcess<'a> {
        ChildPipedProcess {
            name: name,
            args: args,
            output: vec![],
        }
    }

    pub fn process_output(&mut self) -> &Self {

        let child_head = Command::new(self.name)
        .args(self.args)
        .stdin(Stdio::from (
            raw_stdin()
                .expect("couldn't get the std in handle")
            )
        )
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldn't create the child");

        drop(self.args);

        let mut header_output = child_head
            .wait_with_output()
            .expect("child-out").stdout;

        self.output.append(&mut header_output);

        // self.output.as_mut::<&mut Vec<u8>>().append(&mut header_output);

        self
    }

    pub fn output_to_str(&self) -> String {
        let out = String::from_utf8(self.output.to_vec()) 
            .expect("something went wrong with converting u8 vec to str");

        out
    }

}