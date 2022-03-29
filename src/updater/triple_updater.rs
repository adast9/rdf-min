use std::{collections::HashMap, hash::Hash};

use crate::{parser::{triple::Triple, Stuff}, util::generate_new_id};

use super::funcs::{get_key_by_value, remove_parent};

pub fn update_triples(stuff: &mut Stuff, snodes: &Vec<Vec<u32>>) {
    for snode in snodes {
        let new_key = generate_new_id(&stuff.dict);

        // dict
        let mut new_dict_key = String::new();
        for node in snode {
            let dict_key = get_key_by_value(&stuff.dict, node);
            new_dict_key.push_str(&dict_key);
            new_dict_key.push_str("_");
            
            stuff.dict.remove(&dict_key);
        }
        stuff.dict.insert(new_dict_key, new_key);

        // triples + index_map
        for triple in &mut stuff.triples {
            update_id(triple, snode, &mut stuff.index_map, new_key);
        }
        let mut m: Vec<u32> = Vec::new();
        let mut first_sn: u32;
        let mut is_first_sn: bool = true;
        for node in snode {
            if stuff.supernodes.contains_key(node) {
                m.extend(stuff.supernodes.get(node).unwrap().clone());
                for singlenode in stuff.supernodes.get(node).unwrap() {
                    remove_parent(&mut stuff.nodes, singlenode);
                }
                stuff.supernodes.remove(node);
            }
        }
        stuff.supernodes.insert(new_key, m);
    }
}

fn update_id(triple: &mut Triple, snode: &Vec<u32>, index_map: &mut HashMap<u32, [usize; 2]>, new_key: u32) {
    let mut value: [usize; 2] = [0, 0];
    let mut is_inserted = false;
    for node in snode {
        if &triple.sub == node { 
            is_inserted = true;

            triple.sub = new_key; 

            value = index_map.get(node).unwrap().clone();
            index_map.remove(node);
        } 
        if &triple.obj == node { 
            is_inserted = true;

            triple.obj = new_key;

            value = index_map.get(node).unwrap().clone();
            index_map.remove(node);
        } 
        if is_inserted == true { index_map.insert(new_key, value); }
    }
}