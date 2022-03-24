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
        let changes = clique_updater::get_changes(
            index_map,
            &ins,
            source_clique,
            target_clique,
            nodes,
            supernodes,
        );
        let snodes = get_super_nodes(changes, source_clique, target_clique, index_map);
    }
}

fn get_super_nodes(
    changes: Vec<Clique_Change>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    index_map: &mut HashMap<u32, [usize; 2]>,
) -> Vec<Vec<u32>> {
    if changes.len() == 1 {
        return handle_clique_change(changes[0], source_clique, target_clique, index_map);
    }

    let cc1 = handle_clique_change(changes[0], source_clique, target_clique, index_map);
    let cc2 = handle_clique_change(changes[1], source_clique, target_clique, index_map);

    let mut done: Vec<Vec<u32>> = Vec::new();
    let mut marks: Vec<[usize; 2]> = Vec::new();

    for (i, sn1) in cc1.iter().enumerate() {
        let mut used = false;
        for (j, sn2) in cc2.iter().enumerate() {
            if intersects(sn1, sn2) {
                marks.push([i, j]);
                used = true;
            }
        }
        if !used {
            done.push(sn1.clone());
        }
    }

    // get the unmarked supernodes first

    // merge marked supernodes
    for (i, m) in marks.iter().enumerate() {
        //
    }
    return done;
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
        if intersect.len() >= 2 {
            super_nodes.push(intersect);
        }
    }
    return super_nodes;
}

fn intersects(v1: &Vec<u32>, v2: &Vec<u32>) -> bool {
    for n in v1 {
        if v2.contains(&n) {
            return true;
        }
    }
    return false;
}
