use crate::classes::{
    clique::{CliqueChange, CliqueCollection},
    dataset::Dataset,
    meta::Meta,
    triple::Triple,
};

mod all_known;
// mod all_unknown;
// mod pred_unknown;

pub fn get_changes(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<CliqueChange> {
    prepare_triple(triple, dataset, meta, sc, tc);
    return all_known::insert_triple(triple, dataset, meta, sc, tc);
}

fn prepare_triple(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) {
    let (sub_known, pred_known, obj_known) = are_they_known(triple, meta, sc);

    if !sub_known {
        sc.add_node_to_empty_clique(&triple.sub);
        tc.add_node_to_empty_clique(&triple.sub);
        meta.new_node(triple, true);
    } else {
        meta.add_outgoing(triple);
    }

    if !obj_known {
        sc.add_node_to_empty_clique(&triple.obj);
        tc.add_node_to_empty_clique(&triple.obj);
        meta.new_node(triple, false);
    } else {
        meta.add_incoming(triple);
    }

    if !pred_known {
        sc.new_pred(&triple.pred);
        tc.new_pred(&triple.pred);
    }

    dataset.add_triple(triple.clone(), &meta);
}

fn are_they_known(triple: &Triple, meta: &Meta, clique: &CliqueCollection) -> (bool, bool, bool) {
    let sub_known = meta.contains(&triple.sub);
    let obj_known = meta.contains(&triple.obj);
    let pred_known = clique.contains_pred(&triple.pred);
    return (sub_known, pred_known, obj_known);
}
