#![feature(convert)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokenizer::Tokenizer;
use interpreter::Interpreter;

mod tokenizer;
mod operations;
mod interpreter;

fn main() {
    let mut arguments = env::args();
    arguments.next();
    match arguments.next() {
        Some(filename) => {
            match File::open(&Path::new(&filename)) {
                Ok(mut file) => {
                    let mut string = String::new();
                    file.read_to_string(&mut string);
                    let content = string.replace("\n", " ");
                    let words = content.split(' ').collect::<Vec<&str>>();
                    let mut tokenizer = Tokenizer::new(words);
                    let ops = tokenizer.tokenize();
                    let mut interpreter = Interpreter::new(ops);
                    interpreter.run();
                },
                Err(error) => panic!("Could not open file {}", filename),
            }
        },
        None => println!("Usage: groot file.groot"),
    }
}
