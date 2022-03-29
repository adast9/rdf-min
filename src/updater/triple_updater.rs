use std::collections::HashMap;

use crate::{parser::{triple::Triple, Stuff}, util::generate_new_id};

pub fn update_triples(stuff: &mut Stuff, snodes: &Vec<Vec<u32>>) {
    for snode in snodes {
        let new_key = generate_new_id(&stuff.dict);
        for triple in &mut stuff.triples {
            update_id(triple, snode, &mut stuff.index_map, new_key);
        }
    }
}

fn update_id(triple: &mut Triple, snode: &Vec<u32>, index_map: &mut HashMap<u32, [usize; 2]>, new_key: u32, stuff: &mut Stuff) {
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
        if is_inserted == true { 
            index_map.insert(new_key, value); 
            let m = stuff.dict.get(node).unwrap();

        }
    }
}

