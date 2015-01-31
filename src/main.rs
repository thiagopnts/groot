use std::os;
use std::str::StrExt;
use std::old_io::File;

mod tokenizer;
mod operations;

fn main() {
    let arguments = os::args();
    match arguments.get(1) {
        Some(ref filename) => {
            match File::open(&Path::new(filename)).read_to_string() {
                Ok(s) => {
                    // TODO: I think we can make it simpler
                    let words: Vec<String> = s.replace("\n", " ").split(' ').collect::<Vec<&str>>().iter().map(|s| (*s).to_string()).collect();
                    let mut tokenizer = tokenizer::Tokenizer::new(words);
                    tokenizer.tokenize();
                },
                IoError => panic!("errors"),
            }
        },
        None => println!("Usage: groot file.groot"),
    }
}
