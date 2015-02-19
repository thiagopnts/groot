#![feature(old_io, path, env, os, core)]

use std::env;
use std::str::StrExt;
use std::old_io::File;
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
            match File::open(&Path::new(filename)).read_to_string() {
                Ok(s) => {
                    let content = s.replace("\n", " ");
                    let words = content.split(' ').collect::<Vec<&str>>();
                    let mut tokenizer = Tokenizer::new(words);
                    let ops = tokenizer.tokenize();
                    let mut interpreter = Interpreter::new(ops);
                    interpreter.run();
                },
                Err(error) => panic!("{}", error),
            }
        },
        None => println!("Usage: groot file.groot"),
    }
}
