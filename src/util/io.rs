use io::{BufReader, Error};
use std::fs::remove_file;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

use crate::parser::dict::Dict;
use crate::parser::triple::Triple;

pub fn write_lines(path: &PathBuf, vec: &Vec<String>) -> Result<(), Error> {
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

pub fn write_triples(path: &PathBuf, triples: &Vec<Triple>, dict: &Dict) -> Result<(), Error> {
    if path.exists() {
        remove_file(path)?;
    }

    let mut triple_strings: Vec<String> = Vec::new();

    for triple in triples {
        let sub_string = dict.key_by_value(&triple.sub).unwrap();
        let pred_string = dict.key_by_value(&triple.pred).unwrap();
        let obj_string = dict.key_by_value(&triple.obj).unwrap();

        let triple_string = format!("{} {} {} .", sub_string, pred_string, obj_string);

        if !triple_strings.contains(&triple_string) {
            triple_strings.push(triple_string);
        }
    }

    write_lines(path, &triple_strings)?;

    Ok(())
}

pub fn read_lines<P>(path: &P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    let lines: Vec<_> = BufReader::new(file).lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

pub fn file_exists(path: &PathBuf) -> bool {
    return path.exists();
}
