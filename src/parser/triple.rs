use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const TYPE_STRING: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";

pub struct Triple {
    pub sub: u32,
    pub pred: u32,
    pub obj: u32,
    pub is_type: bool,
}

impl Triple {
    pub fn new(line: &String, dict: &HashMap<String, u32>) -> Self {
        let line_splits: Vec<&str> = line.split(" ").collect();
        let mut is_type_pred = false;

        if line_splits[1] == TYPE_STRING {
            is_type_pred = true;
        }

        Triple {
            sub: *dict.get(line_splits[0]).unwrap(),
            pred: *dict.get(line_splits[1]).unwrap(),
            obj: *dict.get(line_splits[2]).unwrap(),
            is_type: is_type_pred,
        }
    }
}

pub fn push_triples_into_vector(
    triple_path: &str,
    dict: &HashMap<String, u32>,
) -> Result<Vec<Triple>, io::Error> {
    let file = File::open(triple_path)?;
    let reader = BufReader::new(file);

    let mut vector_of_triples: Vec<Triple> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        vector_of_triples.push(Triple::new(&line, dict));
    }
    Ok(vector_of_triples)
}
