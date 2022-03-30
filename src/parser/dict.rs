use crate::util::io;
use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

pub fn parse_dict(
    triple_lines: &Vec<String>,
    dict_path: &PathBuf,
) -> Result<HashMap<String, u32>, Error> {
    // todo: handle blank nodes with a queue

    if io::file_exists(dict_path) {
        Ok(read_dict(dict_path)?)
    } else {
        let (vec, dict) = gen_dict(triple_lines)?;
        io::write_lines(dict_path, &vec)?;
        Ok(dict)
    }
}

fn read_dict(dict_path: &PathBuf) -> Result<HashMap<String, u32>, Error> {
    let mut dict: HashMap<String, u32> = HashMap::new();
    for l in io::read_lines(dict_path)? {
        dict.insert(l, dict.len() as u32);
    }
    Ok(dict)
}

fn gen_dict(triple_lines: &Vec<String>) -> Result<(Vec<String>, HashMap<String, u32>), Error> {
    let mut vec: Vec<String> = Vec::new();
    let mut dict: HashMap<String, u32> = HashMap::new();
    for l in triple_lines {
        let v: Vec<&str> = l.split(' ').collect();
        for i in 0..3 {
            if !dict.contains_key(v[i]) {
                dict.insert(v[i].to_string(), dict.len() as u32);
                vec.push(v[i].to_string());
            }
        }
    }
    Ok((vec, dict))
}

pub fn key_by_val(map: &HashMap<String, u32>, value: u32) -> Option<String> {
    for (key, val) in map.iter() {
        if *val == value {
            return Some(key.to_string());
        }
    }
    None
}
