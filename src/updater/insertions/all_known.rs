use crate::classes::{
    clique::{CliqueChange, CliqueCollection},
    dataset::Dataset,
    meta::Meta,
    triple::Triple,
};

pub fn insert_triple(
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

pub fn insert(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    cc: &mut CliqueCollection,
    other_cc: &mut CliqueCollection,
    is_source: bool,
) -> Option<CliqueChange> {
    let node = if is_source { &triple.sub } else { &triple.obj };

    if meta.has_parent(node) {
        // CASE 1: If node and pred are in the same clique, return None
        let p = meta.get_parent(node).unwrap();
        if cc.in_same_clique(&p, &triple.pred) {
            return None;
        }

        // CASE 2: If node is not in the empty set clique, merge cliques
        if !cc.in_empty_clique(&p) {
            let change = CliqueChange::new_merge(cc, &node, &triple.pred, is_source);
            cc.merge_cliques(&node, &triple.pred);
            return Some(change);
        }
    } else {
        // CASE 1: If node and pred are in the same clique, return None
        if cc.in_same_clique(&node, &triple.pred) {
            return None;
        }

        // CASE 2: If node is not in the empty set clique, merge cliques
        if !cc.in_empty_clique(&node) {
            let change = CliqueChange::new_merge(cc, &node, &triple.pred, is_source);
            cc.merge_cliques(&node, &triple.pred);
            return Some(change);
        }
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
    let prev_p = meta.get_parent(node).unwrap();
    meta.remove_from_supernode(node);
    cc.split_and_move(node, &triple.pred);
    other_cc.split(node, &prev_p);
    dataset.split(node, &prev_p, meta);

    if meta.supernode_len(&prev_p) == 1 {
        meta.to_single_node(&prev_p);
        cc.to_single_node(&prev_p, &node);
        other_cc.to_single_node(&prev_p, &node);
        dataset.to_single_node(&prev_p, &node);
    }

    return Some(CliqueChange::new(
        cc.get_index(&triple.pred),
        vec![*node],
        is_source,
    ));
}
