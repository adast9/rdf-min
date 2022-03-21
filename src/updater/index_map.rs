use std::collections::HashMap;

use crate::clique::Clique;

fn get_index_map(
    source_clique: &Vec<Clique>,
    target_clique: &Vec<Clique>,
) -> HashMap<u32, [u32; 2]> {
    let mut index_map: HashMap<u32, [u32; 2]> = HashMap::new();

    for (i, c) in source_clique.iter().enumerate() {
        for p in c.preds.clone() {
            index_map.insert(p, [i as u32, 0]);
        }
        for n in c.nodes.clone() {
            index_map.insert(n, [i as u32, 0]);
        }
    }

    for (i, c) in target_clique.iter().enumerate() {
        for p in c.preds.clone() {
            let x = index_map.get_mut(&p).unwrap();
            x[1] = i as u32;
        }
    }

    return index_map;
}
