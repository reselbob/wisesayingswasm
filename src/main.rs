use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let fname = "src/fortunes.txt";
    let path = Path::new(fname);
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);
    Ok(())
}