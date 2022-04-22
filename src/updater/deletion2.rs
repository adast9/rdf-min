use crate::{
    models::{
        clique::{CliqueChange, CliqueCollection},
        dataset::Dataset,
        meta::Meta,
        triple::Triple,
    },
    util::set_ops::get_disjoint_sets,
};

fn delete_triple(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<CliqueChange> {
    prepare_triple(triple, meta);

    let mut changes: Vec<CliqueChange> = Vec::new();
    if let Some(change) = delete(triple, dataset, meta, sc, tc, true) {
        changes.push(change);
    }
    if let Some(change) = delete(triple, dataset, meta, tc, sc, false) {
        changes.push(change);
    }
    return changes;
}

fn delete(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    cc: &mut CliqueCollection,
    other_cc: &mut CliqueCollection,
    is_source: bool,
) -> Option<CliqueChange> {
    let node = if is_source { &triple.sub } else { &triple.obj };
    let n = meta.get_parent(node).unwrap_or(*node);
    let edges = if is_source { &triple.sub } else { &triple.obj };

    // CASE 1: If node is not in a supernode
    if *node == n {
        // Case 1.1: If node has no incoming/outgoing edges left, move to empty clique
        if meta.has_no_edges_left(node, is_source) {
            cc.move_node_to_empty_clique(node);
            return Some(CliqueChange::new(0, vec![*node], is_source));
        }

        // 1.2: Check if the clique has to be split
        let x = cc.get_all_edges(node, is_source, meta);
        let y = get_disjoint_sets(x);

        if y.len() == 1 {
            return None;
        }

        cc.split_by_preds(node, y, meta, is_source);
    }

    return None;
}

fn prepare_triple(triple: &Triple, meta: &mut Meta) {
    meta.remove_incoming(triple);
    meta.remove_outgoing(triple);
}
