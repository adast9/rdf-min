use super::dataset::Dataset;
use super::dict::Dict;
use super::meta::Meta;

const TYPE_STRING: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";

#[derive(Clone, PartialEq)]
pub struct Triple {
    pub sub: u32,
    pub pred: u32,
    pub obj: u32,
    pub is_type: bool,
}

impl Triple {
    pub fn new(sub: u32, pred: u32, obj: u32, is_type: bool) -> Self {
        Self {
            sub,
            pred,
            obj,
            is_type,
        }
    }

    pub fn from_string(line: &String, dict: &mut Dict) -> Self {
        let words: Vec<&str> = line.split(" ").collect();
        let sub_str = String::from(words[0]);
        let pred_str = String::from(words[1]);
        let obj_str = String::from(words[2]);

        Triple {
            sub: dict.add_if_new(&sub_str),
            pred: dict.add_if_new(&pred_str),
            obj: dict.add_if_new(&obj_str),
            is_type: pred_str == TYPE_STRING,
        }
    }

    pub fn to_string(&self, dataset: &Dataset) -> String {
        let sub_string = dataset.key_by_value(&self.sub).unwrap();
        let pred_string = dataset.key_by_value(&self.pred).unwrap();
        let obj_string = dataset.key_by_value(&self.obj).unwrap();
        return format!("{} {} {} .", sub_string, pred_string, obj_string);
    }

    pub fn rename_node(&mut self, old: &u32, new: &u32) {
        if self.sub == *old {
            self.sub = *new;
        }
        if self.obj == *old {
            self.obj = *new;
        }
    }
}

pub struct TripleCollection {
    pub data_triples: Vec<Triple>,
}

impl TripleCollection {
    pub fn new(triples: Vec<String>, dict: &mut Dict, meta: &mut Meta, add_type: bool) -> Self {
        let mut data_triples: Vec<Triple> = Vec::new();

        for l in triples {
            let t = Triple::from_string(&l, dict);
            if t.is_type {
                if add_type {
                    meta.add_type(&t.sub, &t.obj);
                }
            } else {
                data_triples.push(t);
            }
        }

        Self { data_triples }
    }

    pub fn add_data_triple(&mut self, triple: &Triple) {
        if !self.data_triples.contains(triple) {
            self.data_triples.push(triple.clone());
        }
    }

    pub fn remove_triple(&mut self, triple: &Triple) {
        self.data_triples.retain(|t| t != triple);
    }
}
