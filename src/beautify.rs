use std::str::{Lines};
use colored::{Colorize, ColoredString};
use term_size;

pub enum State {
    Header,
    Body,
}

pub struct BeautifyLines {
    state: State,
    string: String,
    start_ind: usize,
    grep_word: Option<String>,
}


impl BeautifyLines {

    pub fn new(state: State, string: String) -> BeautifyLines {
        BeautifyLines {
            state: state,
            string: string,
            start_ind: 0,
            grep_word: None,
        }

    }

    pub fn set_start_ind(&mut self, ind: usize) -> &Self{
        self.start_ind = ind;

        self
    }

    pub fn set_grep_blinker(&mut self, grep_word: String) -> &Self{
        self.grep_word = Some(grep_word);

        self
    }


    pub fn print (&self) {

        match self.state {
            State::Header => self.print_header(),
            State::Body => self.print_body(),
        }

    }


    fn print_header(&self) {

        let vec_str = Self::limit_lines(self.string.lines());

        let header_str = vec_str.get(0).expect("We need at least one line of output to populate header");

        print!("{}\n", header_str.on_black().bright_white().bold());
    
        for _ in 0..Self::get_max_len() {
            print!("=")
        }

        println!();

    }


    fn print_body(&self) {

        let vec_str = Self::limit_lines(self.string.lines());

        for i in self.start_ind..vec_str.len() {

            let line = vec_str.get(i).expect("");

            match &self.grep_word {
                Some(grep_word) => {
                    match line.find(grep_word) {
                        Some(start_ind) => {
                            let end_ind = start_ind + grep_word.len();

                            print! (
                                "{}", 
                                Self::custom_colorize(i, &line[0..start_ind])
                            );

                            print! (
                                "{}", 
                                &line[start_ind..end_ind].blink()
                            );

                            print! (
                                "{}", 
                                Self::custom_colorize(i, &line[end_ind..])
                            );
                            
                            println!();

                        },
                        None => {
                            println! (
                                "{}", 
                                Self::custom_colorize(i, line)
                            );
                        },
                    }


                },
                None => {
                    println! (
                        "{}", 
                        Self::custom_colorize(i, line)
                    );
                },
            }
        }
    
        
    }

    fn custom_colorize(i: usize, line: &str) -> ColoredString {

        match i  % 4 {
            0 => line.red(),
            1 => line.bright_green(),
            2 => line.yellow(),
            _ => line.bright_magenta(),
        }
    }


    fn limit_lines(lines: Lines) -> Vec<&str> {

        let max_len = Self::get_max_len();

        let output_new = lines.map(|line| {
            if line.len() < max_len {
                return line
            }
    
            &line[0..max_len]
        }).collect::<Vec<&str>>();

        output_new

    }

    fn get_max_len() -> usize {
        let max_len = term_size::dimensions().expect("Couldn't get the terminal size").0;
        max_len
    }


}