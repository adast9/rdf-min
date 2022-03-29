use crate::parser::{clique::Clique, triple::Triple, Stuff};
mod all_known;
mod all_unknown;
mod pred_unknown;

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
    stuff.triples.push(triple.clone());

    let (sub_known, pred_known, obj_known) = are_they_known(stuff, triple);

    // Case: all_known
    if sub_known && pred_known && obj_known {
        let mut changes: Vec<CliqueChange> = Vec::new();
        if let Some(change) = all_known::insert(stuff, triple, sc, tc, true) {
            changes.push(change);
        }
        if let Some(change) = all_known::insert(stuff, triple, tc, sc, false) {
            changes.push(change);
        }

        return changes;
    }

    // Case: all_unknown
    if !sub_known && !pred_known && !obj_known {
        all_unknown::insert(stuff, triple, sc, tc);
        return vec![];
    }

    // Case: only pred_unknown
    if sub_known && !pred_known && obj_known {
        pred_unknown::insert(stuff, triple, sc, tc);
        return vec![];
    }

    return vec![];
}

fn are_they_known(stuff: &Stuff, triple: &Triple) -> (bool, bool, bool) {
    let sub_is_known =
        stuff.supernodes.contains_key(&triple.sub) || stuff.nodes.contains_key(&triple.sub);
    let pred_is_known =
        stuff.supernodes.contains_key(&triple.pred) || stuff.nodes.contains_key(&triple.pred);
    let obj_is_known =
        stuff.supernodes.contains_key(&triple.obj) || stuff.nodes.contains_key(&triple.obj);

    return (sub_is_known, pred_is_known, obj_is_known);
}
