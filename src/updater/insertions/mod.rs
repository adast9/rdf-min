use crate::parser::{clique::Clique, meta_parser::NodeInfo, triple::Triple, Stuff};

use super::funcs::index_of_empty_clique;
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
    let (sub_known, pred_known, obj_known) = are_they_known(stuff, triple);

    // HER KOMMER ET VILDT FORSØG PÅ CRAZY!!!!!
    if !sub_known {
        let sc_empty = index_of_empty_clique(sc);
        let tc_empty = index_of_empty_clique(tc);
        sc[sc_empty].nodes.push(triple.sub);
        tc[tc_empty].nodes.push(triple.sub);
        stuff.index_map.insert(triple.sub, [sc_empty, tc_empty]);
        stuff.nodes.insert(
            triple.sub,
            NodeInfo::new(&None, &vec![], &vec![vec![triple.pred, triple.obj]]),
        );
    } else {
        stuff
            .nodes
            .get_mut(&triple.sub)
            .unwrap()
            .outgoing
            .push(vec![triple.pred, triple.obj]);
    }
    if !obj_known {
        let sc_empty = index_of_empty_clique(sc);
        let tc_empty = index_of_empty_clique(tc);
        sc[sc_empty].nodes.push(triple.obj);
        tc[tc_empty].nodes.push(triple.obj);
        stuff.index_map.insert(triple.obj, [sc_empty, tc_empty]);
        stuff.nodes.insert(
            triple.obj,
            NodeInfo::new(&None, &vec![vec![triple.pred, triple.sub]], &vec![]),
        );
    } else {
        stuff
            .nodes
            .get_mut(&triple.obj)
            .unwrap()
            .outgoing
            .push(vec![triple.pred, triple.sub]);
    }
    if !pred_known {
        sc.push(Clique::new(&vec![triple.pred], &vec![]));
        tc.push(Clique::new(&vec![triple.pred], &vec![]));
        stuff
            .index_map
            .insert(triple.pred, [sc.len() - 1, tc.len() - 1]);
    }

    // Dirty dirty
    let mut new_sub = triple.sub;
    let mut new_obj = triple.obj;
    if let Some(p) = stuff.nodes.get(&triple.sub).unwrap().parent {
        new_sub = p;
    }
    if let Some(p) = stuff.nodes.get(&triple.obj).unwrap().parent {
        new_obj = p;
    }
    stuff.triples.push(Triple {
        sub: new_sub,
        pred: triple.pred,
        obj: new_obj,
        is_type: triple.is_type,
    });

    let mut changes: Vec<CliqueChange> = Vec::new();
    if let Some(change) = all_known::insert(stuff, triple, sc, tc, true) {
        changes.push(change);
    }
    if let Some(change) = all_known::insert(stuff, triple, tc, sc, false) {
        changes.push(change);
    }

    return changes;

    // // Case: all_known
    // if sub_known && pred_known && obj_known {
    //     let mut changes: Vec<CliqueChange> = Vec::new();
    //     if let Some(change) = all_known::insert(stuff, triple, sc, tc, true) {
    //         changes.push(change);
    //     }
    //     if let Some(change) = all_known::insert(stuff, triple, tc, sc, false) {
    //         changes.push(change);
    //     }

    //     return changes;
    // }

    // // Case: all_unknown
    // if !sub_known && !pred_known && !obj_known {
    //     all_unknown::insert(stuff, triple, sc, tc);
    //     return vec![];
    // }

    // // Case: only pred_unknown
    // if sub_known && !pred_known && obj_known {
    //     pred_unknown::insert(stuff, triple, sc, tc);
    //     return vec![];
    // }

    // return vec![];
}

fn are_they_known(stuff: &Stuff, triple: &Triple) -> (bool, bool, bool) {
    let sub_is_known =
        stuff.supernodes.contains_key(&triple.sub) || stuff.nodes.contains_key(&triple.sub);
    let obj_is_known =
        stuff.supernodes.contains_key(&triple.obj) || stuff.nodes.contains_key(&triple.obj);

    let pred_is_known = stuff.index_map.contains_key(&triple.pred);

    return (sub_is_known, pred_is_known, obj_is_known);
}
