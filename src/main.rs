use std::os;
use std::str::StrExt;
use std::old_io::File;
use tokenizer::Tokenizer;
use interpreter::Interpreter;

mod tokenizer;
mod operations;
mod interpreter;

fn main() {
    let arguments = os::args();
    match arguments.get(1) {
        Some(ref filename) => {
            match File::open(&Path::new(filename)).read_to_string() {
                Ok(s) => {
                    // TODO: I think we can make it simpler
                    let words: Vec<String> = s.replace("\n", " ").split(' ').collect::<Vec<&str>>().iter().map(|s| (*s).to_string()).collect();
                    let mut tokenizer = Tokenizer::new(words);
                    let ops = tokenizer.tokenize();
                    let mut interpreter = Interpreter::new(ops);
                    interpreter.run();
                },
                IoError => panic!("io error"),
            }
        },
        None => println!("Usage: groot file.groot"),
    }
}
