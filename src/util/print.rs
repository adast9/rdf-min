use crate::parser::clique::Clique;
use crate::parser::dict::key_by_val;
use std::collections::HashMap;

pub fn cliques(cliques: &Vec<Clique>) {
    for clique in cliques {
        println!("{:?}: {:?}", clique.preds, clique.nodes);
    }
}

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
