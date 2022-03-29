use crate::parser::clique::Clique;
use crate::parser::triple::Triple;
use crate::parser::Stuff;

use super::all_known;

#[derive(Clone)]
pub struct CliqueChange {
    pub clique_index: usize,
    pub new_nodes: Vec<u32>,
    pub is_source: bool,
}

impl CliqueChange {
    pub fn new(clique_index: usize, new_nodes: Vec<u32>, is_source: bool) -> Self {
        Self {
            clique_index,
            new_nodes,
            is_source,
        }
    }
}

pub fn get_changes(
    stuff: &mut Stuff,
    triple: &Triple,
    sc: &mut Vec<Clique>,
    tc: &mut Vec<Clique>,
) -> Vec<CliqueChange> {
    // todo: handle multiple triple-type cases - right know, only all_known is supported

    let mut changes: Vec<CliqueChange> = Vec::new();

    if let Some(change) = all_known::insert(stuff, triple, sc, tc, true) {
        changes.push(change);
    }
    if let Some(change) = all_known::insert(stuff, triple, tc, sc, false) {
        changes.push(change);
    }

    changes
}
