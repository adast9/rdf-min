use crate::util::{generate_new_id, io};
use std::collections::HashMap;
use std::io::Error;

const TYPE_STRING: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";

#[derive(Clone)]
pub struct Triple {
    pub sub: u32,
    pub pred: u32,
    pub obj: u32,
    pub is_type: bool,
}

impl Triple {
    pub fn new(line: &String, dict: &mut HashMap<String, u32>) -> Self {
        let line_splits: Vec<&str> = line.split(" ").collect();
        let mut is_type_pred = false;

        if line_splits[1] == TYPE_STRING {
            is_type_pred = true;
        }

        if !dict.contains_key(line_splits[0]) {
            dict.insert(line_splits[0].to_string(), generate_new_id(dict));
        }
        if !dict.contains_key(line_splits[1]) {
            dict.insert(line_splits[1].to_string(), generate_new_id(dict));
        }
        if !dict.contains_key(line_splits[2]) {
            dict.insert(line_splits[2].to_string(), generate_new_id(dict));
        }

        Triple {
            sub: *dict.get(line_splits[0]).unwrap(),
            pred: *dict.get(line_splits[1]).unwrap(),
            obj: *dict.get(line_splits[2]).unwrap(),
            is_type: is_type_pred,
        }
    }
}

pub fn get_triples(
    triple_lines: &Vec<String>,
    update_path: &str,
    dict: &mut HashMap<String, u32>,
) -> Result<(Vec<Triple>, Vec<Triple>, Vec<Triple>), Error> {
    // todo : multi thread
    let triples = get_current_triples(&triple_lines, dict)?;
    let (additions, deletions) = get_update_triples(update_path, dict)?;

    Ok((triples, additions, deletions))
}

fn get_current_triples(
    triple_lines: &Vec<String>,
    dict: &mut HashMap<String, u32>,
) -> Result<Vec<Triple>, Error> {
    let mut triples: Vec<Triple> = Vec::new();

    for l in triple_lines {
        triples.push(Triple::new(&l, dict));
    }
    Ok(triples)
}

fn get_update_triples(
    update_path: &str,
    dict: &mut HashMap<String, u32>,
) -> Result<(Vec<Triple>, Vec<Triple>), Error> {
    // todo: validate update triples
    let mut additions: Vec<Triple> = Vec::new();
    let mut deletions: Vec<Triple> = Vec::new();

    for l in io::read_lines(update_path)? {
        // if l starts with '-', then it is a deletion
        let ch = l.chars().next().unwrap();

        if ch == '-' {
            let mut l = l;
            l.remove(0);
            deletions.push(Triple::new(&l, dict));
        } else {
            additions.push(Triple::new(&l, dict));
        }
    }
    Ok((additions, deletions))
}
