use super::dict::Dict;

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
        //todo: CLEAN UP THIS SHIT THAT IS WRONG AND DOESN'T WORK PROPERLY AND ISN'T DRY AT ALL
        let line_splits: Vec<&str> = line.split(" ").collect();
        let mut is_type_pred = false;

        if line_splits[1] == TYPE_STRING {
            is_type_pred = true;
        }

        let sub_str = String::from(line_splits[0]);
        let pred_str = String::from(line_splits[1]);
        let obj_str = String::from(line_splits[2]);

        if !dict.contains(&sub_str) {
            dict.add(&sub_str);
        }
        if !dict.contains(&pred_str) {
            dict.add(&pred_str);
        }
        if !dict.contains(&obj_str) {
            dict.add(&obj_str);
        }

        Triple {
            sub: *dict.get(&sub_str).unwrap(),
            pred: *dict.get(&pred_str).unwrap(),
            obj: *dict.get(&obj_str).unwrap(),
            is_type: is_type_pred,
        }
    }

    pub fn to_string(&self, dict: &Dict) -> String {
        let sub = dict.key_by_value(&self.sub).unwrap();
        let pred = dict.key_by_value(&self.pred).unwrap();
        let obj = dict.key_by_value(&self.obj).unwrap();

        let mut line = String::new();
        line.push_str(&sub);
        line.push(' ');
        line.push_str(&pred);
        line.push(' ');
        line.push_str(&obj);
        line.push_str(" .");

        return line;
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
    pub type_triples: Vec<Triple>,
}

impl TripleCollection {
    pub fn new(triples: Vec<String>, dict: &mut Dict) -> Self {
        let mut data_triples: Vec<Triple> = Vec::new();
        let mut type_triples: Vec<Triple> = Vec::new();

        for l in triples {
            let t = Triple::from_string(&l, dict);
            if t.is_type {
                type_triples.push(t);
            } else {
                data_triples.push(t);
            }
        }

        Self {
            data_triples,
            type_triples,
        }
    }

    pub fn add_data_triple(&mut self, triple: &Triple) {
        if !self.data_triples.contains(triple) {
            self.data_triples.push(*triple);
        }
    }
}
