const TYPE_STRING: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";

#[derive(Clone)]
pub struct Triple {
    pub sub: u32,
    pub pred: u32,
    pub obj: u32,
    pub is_type: bool,
}

impl Triple {
    pub fn new(line: &String, dict: &mut Dict) -> Self {
        let line_splits: Vec<&str> = line.split(" ").collect();
        let mut is_type_pred = false;

        if line_splits[1] == TYPE_STRING {
            is_type_pred = true;
        }

        let sub_str = String::from(line_splits[0]);
        let pred_str = String::from(line_splits[1]);
        let obj_str = String::from(line_splits[2]);

        if !dict.contains2(&sub_str) {
            dict.add2(&sub_str);
        }
        if !dict.contains2(&pred_str) {
            dict.add2(&pred_str);
        }
        if !dict.contains2(&obj_str) {
            dict.add2(&obj_str);
        }

        Triple {
            sub: *dict.get2(&sub_str).unwrap(),
            pred: *dict.get2(&pred_str).unwrap(),
            obj: *dict.get2(&obj_str).unwrap(),
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
}
