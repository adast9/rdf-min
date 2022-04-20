use crate::classes::{
    clique::{CliqueChange, CliqueCollection},
    dataset::Dataset,
    meta::Meta,
    triple::Triple,
};

pub fn get_changes(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<CliqueChange> {
    prepare_triple(triple, dataset, meta, sc, tc);
    return insert_triple(triple, dataset, meta, sc, tc);
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

fn insert_triple(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<CliqueChange> {
    let mut changes: Vec<CliqueChange> = Vec::new();
    if let Some(change) = insert(triple, dataset, meta, sc, tc, true) {
        changes.push(change);
    }
    if let Some(change) = insert(triple, dataset, meta, tc, sc, false) {
        changes.push(change);
    }
    return changes;
}

fn insert(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    cc: &mut CliqueCollection,
    other_cc: &mut CliqueCollection,
    is_source: bool,
) -> Option<CliqueChange> {
    let node = if is_source { &triple.sub } else { &triple.obj };
    let n = meta.get_parent(node).unwrap_or(*node);

    // CASE 1: If node and pred are in the same clique, return None
    if cc.in_same_clique(&n, &triple.pred) {
        return None;
    }

    // CASE 2: If node is not in the empty set clique, merge cliques
    if !cc.in_empty_clique(&n) {
        let change = CliqueChange::new_merge(cc, &n, &triple.pred, is_source);
        cc.merge_cliques(&n, &triple.pred);
        return Some(change);
    }

    // CASE 3: If node is not in a supernode, but in the empty set clique, move node to pred clique
    if !meta.has_parent(node) {
        cc.move_node(&node, &triple.pred);
        return Some(CliqueChange::new(
            cc.get_index(&triple.pred),
            vec![*node],
            is_source,
        ));
    }

    // CASE 4: If node is a supernode AND in the empty set clique, split node from its supernode
    split(node, &triple.pred, cc, other_cc, dataset, meta);

    return Some(CliqueChange::new(
        cc.get_index(&triple.pred),
        vec![*node],
        is_source,
    ));
}

fn split(
    node: &u32,
    target_pred: &u32,
    cc: &mut CliqueCollection,
    other_cc: &mut CliqueCollection,
    dataset: &mut Dataset,
    meta: &mut Meta,
) {
    let parent = meta.get_parent(node).unwrap();
    let to_single = meta.remove_from_supernode(node);

    cc.split_and_move(node, target_pred);
    other_cc.split(node, &parent);
    dataset.split(node, &parent, meta, to_single);

    if to_single {
        let n = meta.get_supernode(&parent).unwrap()[0];
        meta.to_single_node(&parent);
        cc.to_single_node(&parent, &n);
        other_cc.to_single_node(&parent, &n);
        dataset.to_single_node(&parent, &n);
    }
}
