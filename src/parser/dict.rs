use crate::util::io;
use crate::Config;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Error;
use std::path::PathBuf;

pub struct Dict {
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

    pub fn add2(&mut self, key: &String) -> u32 {
        if key.is_empty() {
            let id = self.next_id();
            self.queue.push_back(id);
            return id;
        }

        if let Some(id) = self.queue.pop_front() {
            self.dict.insert(key.to_string(), id);
            return id;
        } else {
            let id = self.next_id();
            self.dict.insert(key.to_string(), id);
            return id;
        }
    }

    pub fn remove2(&mut self, key: &String) {
        if !self.contains2(key) {
            panic!("[remove] Key {} not found in dict.", key);
        };
        let id = self.dict.get(key).unwrap().clone();
        self.dict.remove(key);
        self.queue.push_back(id);
    }

    pub fn contains2(&self, key: &String) -> bool {
        return self.dict.contains_key(key);
    }

    pub fn get2(&self, key: &String) -> Option<&u32> {
        return self.dict.get(key);
    }

    pub fn key_by_value(&self, value: &u32) -> Option<String> {
        return self
            .dict
            .iter()
            .find(|(_, v)| **v == *value)
            .map(|(k, _)| k.to_string());
    }

    pub fn update_key(&mut self, new: &String, old: &String) {
        let val = self.get2(old).unwrap().clone();
        self.dict.remove(old);
        self.dict.insert(new.to_string(), val);
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.resize(self.total_len(), String::new());

        for (k, v) in &self.dict {
            vec[*v as usize - 1] = k.to_string();
        }
        return vec;
    }

    fn total_len(&self) -> usize {
        return self.dict.len() + self.queue.len();
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
    let mut dict = Dict::new();
    for l in io::read_lines(dict_path)? {
        dict.add2(&l);
    }
    Ok(dict)
}

fn gen_dict(triple_lines: &Vec<String>) -> Result<Dict, Error> {
    let mut dict = Dict::new();
    for l in triple_lines {
        let v: Vec<&str> = l.split(' ').collect();
        for i in 0..3 {
            let str = v[i].to_string();
            if !dict.contains2(&str) {
                dict.add2(&str);
            }
        }
    }
    Ok(dict)
}
