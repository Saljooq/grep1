
use std::env;
mod child;
mod beautify;

use child::ChildPipedProcess;
use beautify::BeautifyLines;

fn main() {

    
    let header_str = ChildPipedProcess::new("head", &["-n1"])
        .process_output()
        .output_to_str();

    BeautifyLines::new(beautify::State::Header,header_str).print();
    
    let args = env::args()
        .collect::<Vec<_>>();
    
    match args.get(1) {
        Some(arg) => {
            let grep_str = ChildPipedProcess::new("grep", &[arg]).process_output().output_to_str();
            BeautifyLines::new(beautify::State::Body, grep_str).print();
        },
        None => {
            println!("To see the grep match add an argument, for now printing all...");

            let grep_str = ChildPipedProcess::new("cat", &[]).process_output().output_to_str();
            BeautifyLines::new(beautify::State::Body, grep_str).print();
        }
    }
        

}
