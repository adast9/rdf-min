use crate::parser::clique::Clique;
use crate::parser::triple::Triple;
use std::collections::HashMap;

use super::index_map;

pub struct Clique_Change {
    clique_index: usize,
    new_nodes: Vec<u32>,
    is_source: bool,
}

impl Clique_Change {
    pub fn new(clique_index: usize, new_nodes: Vec<u32>, is_source: bool) -> Self {
        Self {
            clique_index,
            new_nodes,
            is_source,
        }
    }
}

pub fn get_changes(
    index_map: &mut HashMap<u32, [usize; 2]>,
    triple: &Triple,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
) -> Vec<Clique_Change> {
    let mut changes: Vec<Clique_Change> = Vec::new();

    if let Some(change) = get_clique_change(index_map, triple, source_clique, true) {
        changes.push(change);
    }
    if let Some(change) = get_clique_change(index_map, triple, target_clique, false) {
        changes.push(change);
    }

    changes
}

fn get_clique_change(
    index_map: &mut HashMap<u32, [usize; 2]>,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    is_source: bool,
) -> Option<Clique_Change> {
    // todo: FIX [0]
    let pred_index = index_map.get(&triple.pred).unwrap()[0];
    let node_index: usize;
    if is_source {
        node_index = index_map.get(&triple.sub).unwrap()[0];
    } else {
        node_index = index_map.get(&triple.obj).unwrap()[1];
    }

    if node_index != pred_index {
        // get cliques to merge
        let mut node_clique = clique[node_index].clone();
        let mut pred_clique = clique[pred_index].clone();

        // Add change
        let change: Clique_Change;
        if node_clique.nodes.len() < pred_clique.nodes.len() {
            change = Clique_Change::new(node_index, node_clique.nodes.clone(), is_source);
        } else {
            change = Clique_Change::new(node_index, pred_clique.nodes.clone(), is_source);
        }

        // merge pred_clique into node_clique and empty pred_clique
        // (Vi kan ikke bare fjerne pred_clique, da vores værdier i index_map bliver fucked)
        node_clique.merge(&pred_clique);
        clique[node_index] = node_clique;
        clique[pred_index] = Clique::empty();

        // Update index_map
        index_map::update_index_map(index_map, &pred_clique, node_index, is_source);

        Some(change)
    } else {
        None
    }
}
