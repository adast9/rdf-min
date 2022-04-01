use crate::parser::clique::Clique;
use crate::parser::dict::Dict;
use crate::parser::triple::Triple;

pub fn cliques_string(cliques: &Vec<Clique>, dict: &Dict) {
    for clique in cliques {
        let mut pred_strings: Vec<String> = Vec::new();
        let mut node_strings: Vec<String> = Vec::new();

        for pred in &clique.preds {
            pred_strings.push(dict.key_by_value(pred).unwrap());
        }
        for node in &clique.nodes {
            node_strings.push(dict.key_by_value(node).unwrap());
        }

        println!("{:?}: {:?}", pred_strings, node_strings);
    }
}

pub fn triples_string(triples: &Vec<Triple>, dict: &Dict) {
    for triple in triples {
        let sub_string = dict.key_by_value(&triple.sub).unwrap();
        let pred_string = dict.key_by_value(&triple.pred).unwrap();
        let obj_string = dict.key_by_value(&triple.obj).unwrap();

        println!("{} {} {}", sub_string, pred_string, obj_string);
    }
}
