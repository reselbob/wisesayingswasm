use rand::seq::IteratorRandom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[no_mangle]
pub extern "C" fn getWiseSaying() -> String {
    let str = include_str!("fortunes.txt");
    let mut lines = str.lines();

    let mut line = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
    return line.to_string();
}
