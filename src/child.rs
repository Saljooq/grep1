use std::process::{Command, Stdio};

use tokio_file_unix::raw_stdin;

pub struct ChildPipedProcess<'a> {
    name: String,
    args: &'a [&'a str],
    output: Vec<u8>,
}

impl <'a> ChildPipedProcess <'a>{


    fn new(name: String, args: &[&str]) -> ChildPipedProcess<'a> {
        ChildPipedProcess {
            name: name,
            args: args,
            output: vec![],
        }
    }

    fn process_output(&self) -> &Self {

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

        let header_output = child_head
            .wait_with_output()
            .expect("child-out").stdout;
        
        self.output = header_output;

        self
        // let header_str = String::from_utf8(header_output) 
        // .expect("something went wrong with converting u8 vec to str");

}

fn output_to_str(&self) -> String {
    let out = String::from_utf8(self.output) 
        .expect("something went wrong with converting u8 vec to str");

    out
}



}