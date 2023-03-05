// use std::io::{Read, Write};
use std::process::Stdio;
use std::{process::Command};
use std::env;
use term_size;
use colored::Colorize;
use tokio_file_unix::{raw_stdin};

mod child;
use child::ChildPipedProcess;


// SOLUTION: https://stackoverflow.com/questions/49218599/write-to-child-process-stdin-in-rust
fn main() {


    let child_head = Command::new("head")
        .arg("-n1")
        .stdin(Stdio::from (
            raw_stdin()
                .expect("couldn't get the std in handle")
            )
        )
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldn't create the child");

    let header_output = child_head
        .wait_with_output()
        .expect("child-out").stdout;
    
    let header_str = String::from_utf8(header_output) 
    .expect("something went wrong with converting u8 vec to str");

    let args = env::args()
        .collect::<Vec<_>>();
    
    let grep_arg = args
        .get(1)
        .expect("You need atleast one argument for grep");



    let child_grep = Command::new("grep")
    .arg(grep_arg)
    .stdin(Stdio::from (
        raw_stdin()
            .expect("couldn't get the std in handle")
        )
    )
    .stdout(Stdio::piped())
    .spawn()
    .expect("couldn't create the child");

    let grep_output = child_grep
        .wait_with_output()
        .expect("child-out").stdout;

    let grep_str = String::from_utf8(grep_output)
    .expect("something went wrong with converting u8 vec to str");



    
    let lines = grep_str.lines();

    let max_len = term_size::dimensions().expect("Couldn't get the terminal size").0;

    let output_new = lines.map(|line| {
        if line.len() < max_len {
            return line
        }

        &line[0..max_len]
    }).collect::<Vec<&str>>();

    // println!("ps output: \n");
    print!("{}", header_str.on_black().bright_white().bold());
    
    for _ in 0..max_len {
        print!("=")
    }

    println!();

    for i in 0..output_new.len() {

        let line = output_new.get(i).expect("");

        let colored_line = match i  % 4 {
            0 => line.red(),
            1 => line.bright_green(),
            2 => line.yellow(),
            _ => line.bright_magenta(),
        };


        println! (
            "{}", 
            colored_line
        );
    }


}
