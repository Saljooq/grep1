use std::io::Read;
use std::{process::Command};
use std::env;
use term_size;
use colored::Colorize;
use tokio_file_unix::{raw_stdin};

fn main() {


    let mut input = raw_stdin().expect("No input piped in");



    let mut input_str= vec![];

    input.read_to_end(&mut input_str).expect("something wend reading reading file");

    let input = String::from_utf8(input_str)
    .expect("something went wrong with converting input data piped in, to a string");

    // println!("{}", &input);


    let args: Vec<String> = env::args().collect();


    let mut binding = Command::new("sh");
    let ps = binding
        .arg("-c")
        .arg(format!("echo \"{}\" | head -n1 && echo \"{}\"  | grep ", &input, &input).to_string() + args.get(1).expect("argument expected"));


    let output = String::from_utf8 (
        ps
        .output()
        .expect("something went wrong with using ps")
        .stdout
    )
    .expect("Could not covert ps res to string");
    
    let lines = output.lines();

    let max_len = term_size::dimensions().expect("Couldn't get the terminal size").0;

    let output_new = lines.map(|line| {
        if line.len() < max_len {
            return line
        }

        &line[0..max_len]
    }).collect::<Vec<&str>>();

    // println!("ps output: \n");
    println!("{}", output_new.get(0).expect("").on_black().bright_white().bold());
    
    for _ in 0..max_len {
        print!("=")
    }

    println!();

    for i in 1..output_new.len() {

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
