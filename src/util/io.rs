use io::{BufReader, Error};
use std::fs::remove_file;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

use crate::classes::dict::Dict;
use crate::classes::meta::Meta;
use crate::classes::triple::Triple;

fn write_lines(path: &PathBuf, vec: &Vec<String>) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    for s in vec {
        writeln!(file, "{}", s)?
    }
    Ok(())
}

pub fn write_triples(path: &PathBuf, triples: &Vec<Triple>, dict: &Dict) -> Result<(), Error> {
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
    Ok(write_lines(path, &triple_strings)?)
}

pub fn write_dict(path: &PathBuf, dict: &Dict) -> Result<(), Error> {
    if path.exists() {
        remove_file(path)?;
    }
    Ok(write_lines(path, &dict.to_strings())?)
}

pub fn write_meta(path: &PathBuf, meta: &Meta) -> Result<(), Error> {
    let data = meta.serialize();
    let file_str = serde_json::to_string(&data)?;
    Ok(write_lines(path, &vec![file_str])?)
}

pub fn read_lines<P>(path: &P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    let lines: Vec<_> = BufReader::new(file).lines().collect::<Result<_, _>>()?;
    Ok(lines)
}
