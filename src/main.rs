extern crate tokenizer;

use std::fs::File;

use tokenizer::tokenize;
use std::io::BufReader;

fn main() {
    let mut f = std::fs::read_to_string("tokenizer/examples/functions.wis").expect("failed to open file");
    for token in tokenize(&f[..]) {
        println!("{:?}", token);
    }
}
