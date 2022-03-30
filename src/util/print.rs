use crate::parser::clique::Clique;
use crate::parser::dict::key_by_val;
use crate::parser::triple::Triple;
use std::collections::HashMap;

pub fn cliques_string(cliques: &Vec<Clique>, dict: &HashMap<String, u32>) {
    for clique in cliques {
        let mut pred_strings: Vec<String> = Vec::new();
        let mut node_strings: Vec<String> = Vec::new();

        for pred in &clique.preds {
            pred_strings.push(key_by_val(dict, *pred).unwrap())
        }
        for node in &clique.nodes {
            node_strings.push(key_by_val(dict, *node).unwrap())
        }

        println!("{:?}: {:?}", pred_strings, node_strings);
    }
}

pub fn triples_string(triples: &Vec<Triple>, dict: &HashMap<String, u32>) {
    for triple in triples {
        let sub_string: String = key_by_val(dict, triple.sub).unwrap();
        let pred_string: String = key_by_val(dict, triple.pred).unwrap();
        let obj_string: String = key_by_val(dict, triple.obj).unwrap();

        println!("{} {} {}", sub_string, pred_string, obj_string);
    }
}
