use crate::parser::clique::Clique;
use std::collections::HashMap;

pub fn get_index_map(
    source_clique: &Vec<Clique>,
    target_clique: &Vec<Clique>,
) -> HashMap<u32, [usize; 2]> {
    let mut index_map: HashMap<u32, [usize; 2]> = HashMap::new();

    for (i, c) in source_clique.iter().enumerate() {
        for p in c.preds.clone() {
            index_map.insert(p, [i, 0]);
        }
        for n in c.nodes.clone() {
            index_map.insert(n, [i, 0]);
        }
    }

    for (i, c) in target_clique.iter().enumerate() {
        for p in c.preds.clone() {
            let x = index_map.get_mut(&p).unwrap();
            x[1] = i;
        }
        for p in c.nodes.clone() {
            let x = index_map.get_mut(&p).unwrap();
            x[1] = i;
        }
    }

    return index_map;
}

pub fn update_index_map(
    index_map: &mut HashMap<u32, [usize; 2]>,
    moved_clique: &Clique,
    new_index: usize,
    is_source: bool,
) {
    let i = if is_source { 0 } else { 1 };
    // Update index_map
    for node in &moved_clique.nodes {
        let arr = index_map.get_mut(node).unwrap();
        arr[i] = new_index;
    }
    for pred in &moved_clique.preds {
        let arr = index_map.get_mut(pred).unwrap();
        arr[i] = new_index;
    }
}
