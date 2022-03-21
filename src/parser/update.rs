use crate::parser::triple::Triple;
use crate::util::io;
use std::collections::HashMap;
use std::io::Error;

pub fn get_update_triples(
    update_path: &str,
    dict: &HashMap<String, u32>,
) -> Result<(Vec<Triple>, Vec<Triple>), Error> {
    // todo: validate update triples

    let mut additions: Vec<Triple> = Vec::new();
    let mut deletions: Vec<Triple> = Vec::new();

    let lines = io::read_lines(update_path)?;
    for l in lines {
        // if l starts with '-', then it is a deletion
        let l = l?;
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
