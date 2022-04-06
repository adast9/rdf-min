use crate::classes::dict::Dict;
use crate::classes::triple::Triple;
use crate::util::io;
use std::io::Error;
use std::path::PathBuf;

pub fn get_triples(
    triple_lines: &Vec<String>,
    update_path: &PathBuf,
    dict: &mut Dict,
) -> Result<(Vec<Triple>, Vec<Triple>, Vec<Triple>), Error> {
    // todo : multi thread
    let triples = get_current_triples(&triple_lines, dict)?;
    let (additions, deletions) = get_update_triples(update_path, dict)?;

    Ok((triples, additions, deletions))
}

fn get_current_triples(triple_lines: &Vec<String>, dict: &mut Dict) -> Result<Vec<Triple>, Error> {
    let mut triples: Vec<Triple> = Vec::new();

    for l in triple_lines {
        triples.push(Triple::new(&l, dict));
    }
    Ok(triples)
}

fn get_update_triples(
    update_path: &PathBuf,
    dict: &mut Dict,
) -> Result<(Vec<Triple>, Vec<Triple>), Error> {
    let mut additions: Vec<Triple> = Vec::new();
    let mut deletions: Vec<Triple> = Vec::new();

    for l in io::read_lines(&update_path)? {
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
