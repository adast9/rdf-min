use self::clique_updater::Clique_Change;
use crate::parser::{clique::Clique, index_map, meta_parser::NodeInfo, triple::Triple};
use std::collections::HashMap;
mod clique_updater;

pub fn run(
    dict: &mut HashMap<String, u32>,
    mut triples: Vec<Triple>,
    insertions: Vec<Triple>,
    deletions: Vec<Triple>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    mut index_map: HashMap<u32, [usize; 2]>,
    mut supernodes: HashMap<u32, Vec<u32>>,
    mut nodes: HashMap<u32, NodeInfo>,
) {
    handle_insersertions(
        insertions,
        &mut index_map,
        source_clique,
        target_clique,
        &mut triples,
        &mut supernodes,
        &mut nodes,
    );
}

fn handle_insersertions(
    insertions: Vec<Triple>,
    index_map: &mut HashMap<u32, [usize; 2]>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    triples: &mut Vec<Triple>,
    supernodes: &mut HashMap<u32, Vec<u32>>,
    nodes: &mut HashMap<u32, NodeInfo>,
) {
    for ins in insertions {
        let changes = clique_updater::get_changes(index_map, &ins, source_clique, target_clique);
        let snodes = get_super_nodes(changes, source_clique, target_clique, index_map);
    }
}

fn get_super_nodes(
    changes: Vec<Clique_Change>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    index_map: &mut HashMap<u32, [usize; 2]>,
) {
    // for each change
    for cc in changes {
        handle_clique_change(cc, source_clique, target_clique, index_map);
    }
}

fn handle_clique_change(
    change: Clique_Change,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    index_map: &mut HashMap<u32, [usize; 2]>,
) -> Vec<Vec<u32>> {
    let mut super_nodes: Vec<Vec<u32>> = Vec::new();

    let c1 = if change.is_source {
        source_clique[change.clique_index].clone()
    } else {
        target_clique[change.clique_index].clone()
    };

    for node in change.new_nodes {
        let c2 = if change.is_source {
            let index = index_map.get(&node).unwrap()[1];
            target_clique[index].clone()
        } else {
            let index = index_map.get(&node).unwrap()[0];
            source_clique[index].clone()
        };

        let intersect = c1.node_intersection(&c2);
        if intersect.len() > 0 {
            super_nodes.push(intersect);
        }
    }
    return super_nodes;
}
