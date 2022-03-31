use crate::util::io;
use crate::Config;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Error;
use std::path::PathBuf;

struct Dict {
    dict: HashMap<String, u32>,
    queue: VecDeque<u32>,
}

impl Dict {
    pub fn new() -> Self {
        return Self {
            dict: HashMap::new(),
            queue: VecDeque::new(),
        };
    }

    pub fn add(&mut self, key: &String) -> u32 {
        if key.is_empty() {
            let id = self.next_id();
            self.queue.push_back(id);
            return id;
        }

        if let Some(id) = self.queue.pop_front() {
            if !self.contains(key) {
                panic!("[add] Key {} not found in dict.", key);
            };
            self.dict.insert(key.to_string(), id);
            return id;
        } else {
            let id = self.next_id();
            self.dict.insert(key.to_string(), id);
            return id;
        }
    }

    pub fn remove(&mut self, key: &String) {
        if !self.contains(key) {
            panic!("[remove] Key {} not found in dict.", key);
        };
        let id = self.dict.get(key).unwrap().clone();
        self.dict.remove(key);
        self.queue.push_back(id);
    }

    pub fn contains(&self, key: &String) -> bool {
        return self.dict.contains_key(key);
    }

    fn next_id(&self) -> u32 {
        return self.dict.len() as u32 + self.queue.len() as u32 + 1;
    }
}

pub fn parse_dict(triple_lines: &Vec<String>, config: &Config) -> Result<Dict, Error> {
    if config.use_fast {
        Ok(gen_dict(triple_lines)?)
    } else {
        let dict_path = config.meta_folder_path.join("dict");
        Ok(read_dict(&dict_path)?)
    }
}

fn read_dict(dict_path: &PathBuf) -> Result<Dict, Error> {
    let dict = Dict::new();
    for l in io::read_lines(dict_path)? {
        dict.add(&l);
    }
    Ok(dict)
}

fn gen_dict(triple_lines: &Vec<String>) -> Result<Dict, Error> {
    let dict = Dict::new();
    for l in triple_lines {
        let v: Vec<&str> = l.split(' ').collect();
        for i in 0..3 {
            let str = v[i].to_string();
            if !dict.contains(&str) {
                dict.add(&str);
            }
        }
    }
    Ok(dict)
}

// fn gen_dict(triple_lines: &Vec<String>) -> Result<(Vec<String>, HashMap<String, u32>), Error> {
//     let mut vec: Vec<String> = Vec::new();
//     let mut dict: HashMap<String, u32> = HashMap::new();
//     for l in triple_lines {
//         let v: Vec<&str> = l.split(' ').collect();
//         for i in 0..3 {
//             if !dict.contains_key(v[i]) {
//                 dict.insert(v[i].to_string(), dict.len() as u32);
//                 vec.push(v[i].to_string());
//             }
//         }
//     }
//     Ok((vec, dict))
// }

pub fn key_by_val(map: &HashMap<String, u32>, value: u32) -> Option<String> {
    for (key, val) in map.iter() {
        if *val == value {
            return Some(key.to_string());
        }
    }
    None
}
