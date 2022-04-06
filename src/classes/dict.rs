use std::collections::{HashMap, VecDeque};

pub struct Dict {
    dict: HashMap<String, u32>,
    queue: VecDeque<u32>,
}

impl Dict {
    /// Creates an empty `Dict`.
    pub fn new() -> Self {
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
