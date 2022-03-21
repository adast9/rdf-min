use io::{BufReader, Error, Lines};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn write_lines(path: &str, vec: &Vec<String>) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;

    for s in vec {
        writeln!(file, "{}", s)?
    }
    Ok(())
}

pub fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

pub fn file_exists(path: &str) -> bool {
    return Path::new(path).exists();
}
