use std::collections::HashMap;
use std::io::Error;
use crate::io_helper::*;

pub(crate) struct Dictionary {
    dict: HashMap<String, u32>
}

impl Dictionary {
    pub fn new(triple_path: &str, dict_path: &str) -> Result<Self, Error> {
        let d = parse_dict(triple_path, dict_path)?;
        Ok (Self {
            dict: d
        })
    }

    pub fn get_dict(&self) -> &HashMap<String, u32> {
        return &self.dict;
    }
    
    pub fn get(&self, key: &str) -> Option<u32> {
        match self.dict.get(key) {
            Some(&v) => Some(v),
            None => None
        }
    }
}

fn parse_dict(triple_path: &str, dict_path: &str) -> Result<HashMap<String, u32>, Error> {
    // todo: handle blank nodes with a queue

    if file_exists(dict_path) {
        Ok(read_dict(dict_path)?)
    } else {
        let (vec, dict) = gen_dict(triple_path)?;
        write_lines(dict_path, &vec)?;
        Ok(dict)
    }
}

fn read_dict(dict_path: &str) -> Result<HashMap<String, u32>, Error> {
    let mut dict: HashMap<String, u32> = HashMap::new();
    let lines = read_lines(dict_path)?;
    
    for l in lines {
        dict.insert(l?, dict.len() as u32);
    }
    Ok(dict)
}

fn gen_dict(triple_path: &str) -> Result<(Vec<String>, HashMap<String, u32>), Error> {
    let mut vec: Vec<String> = Vec::new();
    let mut dict: HashMap<String, u32> = HashMap::new();
    let lines = read_lines(triple_path)?;

    for l in lines {
        let l = l?;
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
