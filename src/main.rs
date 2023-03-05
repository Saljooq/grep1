
use std::{env, io::Read};
mod child;
mod beautify;

use child::ChildPipedProcess;
use beautify::BeautifyLines;
use tokio_file_unix::raw_stdin;

fn main() {

    let mut input_bfr = vec![];
    raw_stdin().expect("").read_to_end(&mut input_bfr).expect("");


    let header_str = ChildPipedProcess::new("head", &["-n1"], Some(&input_bfr))
        .process_output()
        .output_to_str();

    BeautifyLines::new(beautify::State::Header, header_str).print();
    
    let args = env::args()
        .collect::<Vec<_>>();
    
    match args.get(1) {
        Some(arg) => {
            let grep_out = ChildPipedProcess::new("grep", &[arg], Some(&input_bfr)).process_output().output_to_str();
            BeautifyLines::new(beautify::State::Body, grep_out).print();
        },
        None => {
            println!("To see the grep match add an argument, for now printing all...");
            let cat_out = ChildPipedProcess::new("cat", &[], Some(&input_bfr)).process_output().output_to_str();
            BeautifyLines::new(beautify::State::Body, cat_out).set_start_ind(1).print();
        }
    };

        

}
