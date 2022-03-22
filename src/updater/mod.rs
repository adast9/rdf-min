use std::collections::HashMap;

use crate::parser::{clique::Clique, triple::Triple};

mod clique_updater;
mod index_map;
mod meta_parser;

fn run(
    dict: HashMap<String, u32>,
    triples: Vec<Triple>,
    insertions: Vec<Triple>,
    deletions: Vec<Triple>,
    source_clique: Vec<Clique>,
    target_clique: Vec<Clique>,
    index_map: HashMap<u32, [usize; 2]>,
) {
    handle_insersertions(
        insertions,
        index_map,
        &mut source_clique,
        &mut target_clique,
        &mut triples,
    );
}

fn handle_insersertions(
    insertions: Vec<Triple>,
    index_map: HashMap<u32, [usize; 2]>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    triples: &mut Vec<Triple>,
) {
    for ins in insertions {
        let changes = clique_updater::get_changes(
            &mut index_map,
            &ins,
            &mut source_clique,
            &mut target_clique,
        );
    }
}
