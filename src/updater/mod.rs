use std::collections::HashMap;

use crate::parser::{clique::Clique, index_map, meta_parser::NodeInfo, triple::Triple};

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
    }
}
