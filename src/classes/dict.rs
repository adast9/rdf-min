use std::collections::{HashMap, VecDeque};

pub struct Dict {
    dict: HashMap<String, u32>,
    queue: VecDeque<u32>,
}

impl Dict {
    /// Creates an new `Dict`.
    pub fn new(dict_lines: &Vec<String>) -> Self {
        let mut dict = Dict::empty();
        for l in dict_lines {
            dict.add(&l);
        }
        return dict;
    }

    /// Creates an empty `Dict`.
    pub fn empty() -> Self {
        return Self {
            dict: HashMap::new(),
            queue: VecDeque::new(),
        };
    }

    /// Adds a new entry `key` to the `Dict`.
    ///
    /// The id assigned to `key` is returned.
    pub fn add(&mut self, key: &String) -> u32 {
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

    /// Removes the entry `key` from the `Dict`.
    ///
    /// # Panics
    ///
    /// Panics if the `Dict` does not contain `key`.
    pub fn remove(&mut self, key: &String) {
        if !self.contains(key) {
            panic!("[remove] Key {} not found in dict.", key);
        };
        let id = self.dict.get(key).unwrap().clone();
        self.dict.remove(key);
        self.queue.push_back(id);
    }

    /// Returns true if the `Dict` contains an entry `key`.
    pub fn contains(&self, key: &String) -> bool {
        return self.dict.contains_key(key);
    }

    /// Returns a reference to the id of `key`.
    pub fn get(&self, key: &String) -> Option<&u32> {
        return self.dict.get(key);
    }

    /// Returns the key of the value `value`.
    pub fn key_by_value(&self, value: &u32) -> Option<String> {
        return self
            .dict
            .iter()
            .find(|(_, v)| **v == *value)
            .map(|(k, _)| k.to_string());
    }

    /// Updates the key of an entry containing `old` to `new`.
    pub fn update_key(&mut self, new: &String, old: &String) {
        let val = self.get(old).unwrap().clone();
        self.dict.remove(old);
        if !self.dict.contains_key(new) {
            self.dict.insert(new.to_string(), val);
        }
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

    pub fn remove_from_name(&mut self, snode: &u32, node: &u32) {
        let mut snode_string = self.key_by_value(snode).unwrap();
        let node_string = Dict::get_name(&self.key_by_value(node).unwrap());

        let index = snode_string.find(&node_string).unwrap();

        let old_key = snode_string.clone();
        if (index + node_string.len() + 1 == snode_string.len()) {
            snode_string.replace_range((index - 1..index + node_string.len()), "");
        } else {
            snode_string.replace_range((index..index + node_string.len() + 1), "");
        }

        self.update_key(&snode_string, &old_key);
    }

    pub fn get_name(string: &String) -> String {
        let mut name = String::new();
        let mut chars = string.chars();
        while let Some(c) = chars.next_back() {
            if c == '/' {
                break;
            }
            if c == '>' {
                continue;
            }
            name = c.to_string() + &name;
        }
        return name;
    }

    pub fn remove_by_value(&mut self, value: &u32) {
        let key = self.key_by_value(value).unwrap();
        self.remove(&key);
    }

    pub fn contains_value(&self, value: &u32) -> bool {
        for v in self.dict.values() {
            if *v == *value {
                return true;
            }
        }
        return false;
    }
}
