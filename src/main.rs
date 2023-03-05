
use std::{env, io::Read};
mod child;
mod beautify;

use child::ChildPipedProcess;
use beautify::BeautifyLines;
use tokio_file_unix::raw_stdin;

fn main() {

    let mut input_bfr = vec![];
    raw_stdin().expect("").read_to_end(&mut input_bfr).expect("");

    
    // let header_str = ChildPipedProcess::new("head", &["-n1"])
    //     .process_output()
    //     .output_to_str();

    let header_str = ChildPipedProcess::new("head", &["-n1"], Some(&input_bfr))
        .process_output()
        .output_to_str();

    BeautifyLines::new(beautify::State::Header, header_str).print();
    // (beautify::State::Header,header_str).print();
    
    let args = env::args()
        .collect::<Vec<_>>();
    
    let body_str = match args.get(1) {
        Some(arg) => {
            ChildPipedProcess::new("grep", &[arg], Some(&input_bfr)).process_output().output_to_str()
        },
        None => {
            println!("To see the grep match add an argument, for now printing all...");
            ChildPipedProcess::new("cat", &[], Some(&input_bfr)).process_output().output_to_str()
        }
    };

    BeautifyLines::new(beautify::State::Body, body_str).print();
        

}
