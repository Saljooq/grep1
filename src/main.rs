
use std::env;
use term_size;
use colored::Colorize;

mod child;
use child::ChildPipedProcess;


// SOLUTION: https://stackoverflow.com/questions/49218599/write-to-child-process-stdin-in-rust
fn main() {

    
    let header_str = ChildPipedProcess::new("head", &["-n1"])
        .process_output()
        .output_to_str();
    
    let args = env::args()
        .collect::<Vec<_>>();
    
    let grep_arg = args
        .get(1)
        .expect("You need atleast one argument for grep");
    
    let grep_str = ChildPipedProcess::new("grep", &[grep_arg]).process_output().output_to_str();


    let lines = grep_str.lines();

    let max_len = term_size::dimensions().expect("Couldn't get the terminal size").0;

    let output_new = lines.map(|line| {
        if line.len() < max_len {
            return line
        }

        &line[0..max_len]
    }).collect::<Vec<&str>>();

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
