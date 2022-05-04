use crate::{
    models::{
        clique::{CliqueChange, CliqueCollection},
        dataset::Dataset,
        meta::Meta,
        triple::Triple,
    },
    util::set_ops::{get_disjoint_sets, intersection, intersects},
};

pub fn delete_triple(
    triple: &Triple,
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<CliqueChange> {
    prepare_triple(triple, meta, dataset);

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

    // CASE 1: If node is not in a supernode
    if *node == n {
        // Case 1.1: If node has no incoming/outgoing edges left, move to empty clique
        if meta.has_no_edges_left(node, is_source) {
            cc.move_node_to_empty_clique(node);
            return Some(CliqueChange::new(0, vec![*node], is_source));
        }
    }

    // CASE 2: Check if the clique has to be split
    let (mut singlenodes, supernodes, mut edges) = cc.get_all_edges(node, is_source, meta);
    let new_clique_preds = get_disjoint_sets(edges.clone());
    if new_clique_preds.len() == 1 {
        return None;
    }

    remove_supernodes(&supernodes, meta, dataset, cc, other_cc);

    split_clique_by_preds(
        node,
        &mut singlenodes,
        supernodes,
        &mut edges,
        new_clique_preds,
        meta,
        dataset,
        cc,
        other_cc,
    );

    return None;
}

fn prepare_triple(triple: &Triple, meta: &mut Meta, dataset: &mut Dataset) {
    meta.remove_incoming(triple);
    meta.remove_outgoing(triple);
    dataset.remove_triple(triple);
}

fn remove_supernodes(
    supernodes: &Vec<Vec<u32>>,
    meta: &mut Meta,
    dataset: &mut Dataset,
    cc: &mut CliqueCollection,
    other_cc: &mut CliqueCollection,
) {
    for s in supernodes {
        let parent = meta.get_parent(&s[0]).unwrap();
        dataset.remove_supernode(&parent, s.to_vec(), meta);
        cc.remove_supernode(&parent, meta);
        other_cc.remove_supernode(&parent, meta);
        meta.remove_supernode(&parent);
    }
}

pub fn split_clique_by_preds(
    target: &u32,
    singlenodes: &mut Vec<u32>,
    supernodes: Vec<Vec<u32>>,
    edges: &mut Vec<Vec<u32>>,
    clique_preds: Vec<Vec<u32>>,
    meta: &mut Meta,
    dataset: &mut Dataset,
    cc: &mut CliqueCollection,
    other_cc: &mut CliqueCollection,
) {
    let index = cc.get_index(target);

    for preds in clique_preds {
        let mut new_nodes: Vec<u32> = Vec::new();

        for i in (0..singlenodes.len()).rev() {
            if intersects(&edges[i], &preds) {
                new_nodes.push(singlenodes[i]);
                singlenodes.remove(i);
                edges.remove(i);
            }
        }

        cc.new_clique(&preds, &new_nodes);

        for i in (0..supernodes.len()).rev() {
            if let Some(intersec) = intersection(&supernodes[i], &new_nodes) {
                if intersec.len() > 1 {
                    let new_snode = dataset.new_snode(&intersec, meta);
                    meta.new_snode(&intersec, &new_snode);
                    cc.new_snode(&intersec, &new_snode);
                    other_cc.new_snode(&intersec, &new_snode);
                }
            }
        }
    }

    cc.remove_clique_by_index(index);
}
