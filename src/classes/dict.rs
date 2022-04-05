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
