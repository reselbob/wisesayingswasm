//use std::io::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::path::Path;
use rand::seq::IteratorRandom;

fn main() -> std::io::Result<()> {
    let mut str = getWiseSaying();
    println!("{}", str);
    Ok(())
}

fn getWiseSaying() -> String {
    let fname = "src/fortunes.txt";
    let f = File::open(fname).unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", fname, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    let mut line = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
    return line;
}
