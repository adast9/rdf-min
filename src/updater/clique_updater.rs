use crate::parser::clique::Clique;
use crate::parser::index_map;
use crate::parser::meta_parser::NodeInfo;
use crate::parser::triple::Triple;
use std::collections::HashMap;

pub struct Clique_Change {
    clique_index: usize,
    new_nodes: Vec<u32>,
    is_source: bool,
}

impl Clique_Change {
    pub fn new(clique_index: usize, new_nodes: Vec<u32>, is_source: bool) -> Self {
        Self {
            clique_index,
            new_nodes,
            is_source,
        }
    }
}

pub fn get_changes(
    index_map: &mut HashMap<u32, [usize; 2]>,
    triple: &Triple,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    nodes: &mut HashMap<u32, NodeInfo>,
    supernodes: &mut HashMap<u32, Vec<u32>>,
) -> Vec<Clique_Change> {
    let mut changes: Vec<Clique_Change> = Vec::new();

    if let Some(change) = get_clique_change(
        index_map,
        triple,
        source_clique,
        target_clique,
        true,
        nodes,
        supernodes,
    ) {
        changes.push(change);
    }
    if let Some(change) = get_clique_change(
        index_map,
        triple,
        target_clique,
        source_clique,
        false,
        nodes,
        supernodes,
    ) {
        changes.push(change);
    }

    changes
}

fn get_clique_change(
    index_map: &mut HashMap<u32, [usize; 2]>,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    other_clique: &mut Vec<Clique>,
    is_source: bool,
    nodes: &mut HashMap<u32, NodeInfo>,
    supernodes: &mut HashMap<u32, Vec<u32>>,
) -> Option<Clique_Change> {
    let i = if is_source { 0 } else { 1 };
    let node = if is_source { &triple.sub } else { &triple.obj };

    // See if the node in the triple is a supernode
    let supernode = nodes.get(node).unwrap().parent;
    let mut node_is_supernode = false;

    // Get indices in cliques of node and pred
    let pred_index = index_map.get(&triple.pred).unwrap()[i];
    let node_index: usize;
    if let Some(s_node) = supernode {
        node_index = index_map.get(&s_node).unwrap()[i];
        node_is_supernode = true;
    } else {
        node_index = index_map.get(node).unwrap()[i];
    }

    if node_index == pred_index {
        return None;
    } else if node_index != clique.len() - 1 {
        return Some(handle_different_but_not_empty_set(
            clique, node_index, pred_index, is_source, index_map,
        ));
    }

    if node_is_supernode {
        let node_being_split = *node;
        let supernode_id = supernode.unwrap();

        // Remove node_being_split from supernode in meta
        let meta_supernode = supernodes.get_mut(&supernode_id).unwrap();
        meta_supernode.retain(|x| *x != node_being_split);

        // Remove parent from node_being_split in meta
        let n = nodes.get_mut(&node_being_split).unwrap();
        n.parent = None;

        // Insert node_being_split into pred_index in current clique
        clique[pred_index].nodes.push(node_being_split);

        // Insert node_being_split into the same clique as the supernode in the other clique
        let supernode_index_in_other =
            index_map.get(&supernode_id).unwrap()[if is_source { 1 } else { 0 }];
        other_clique[supernode_index_in_other]
            .nodes
            .push(node_being_split);

        // Add to index_map
        index_map.insert(node_being_split, [pred_index, supernode_index_in_other]);

        // After splitting the single node from the supernode, if the supernode now only contains 1 element, we want to remove it
        // We replace the supernode with the single node left behind
        if meta_supernode.len() == 1 {
            let node_left_behind = meta_supernode[0];

            // Remove the supernode in meta
            supernodes.remove(&supernode_id);

            // Remove parent from node_left_behind in meta
            let n = nodes.get_mut(&node_left_behind).unwrap();
            n.parent = None;

            // Remove supernode and replace with node_left_behind in clique
            clique[node_index].nodes.retain(|x| *x != supernode_id);
            clique[node_index].nodes.push(node_left_behind);
            other_clique[supernode_index_in_other]
                .nodes
                .retain(|x| *x != supernode_id);
            other_clique[supernode_index_in_other]
                .nodes
                .push(node_left_behind);
            // Remove supernode from index_map and add node_left_behind in its place
            let x = index_map.get(&supernode_id).unwrap().clone();
            index_map.remove(&supernode_id);
            index_map.insert(node_left_behind, x);
        }

        Some(Clique_Change::new(
            pred_index,
            vec![node_being_split],
            is_source,
        ))
    } else {
        // Move node from empty clique to pred_clique
        clique[pred_index].nodes.push(*node);
        clique[node_index].nodes.retain(|x| *x != *node);

        // Update index_map
        let arr = index_map.get_mut(node).unwrap();
        arr[i] = pred_index;

        Some(Clique_Change::new(pred_index, vec![*node], is_source))
    }
}

fn handle_different_but_not_empty_set(
    clique: &mut Vec<Clique>,
    node_index: usize,
    pred_index: usize,
    is_source: bool,
    index_map: &mut HashMap<u32, [usize; 2]>,
) -> Clique_Change {
    // get cliques to merge
    let mut node_clique = clique[node_index].clone();
    let pred_clique = clique[pred_index].clone();

    // Add change
    let change: Clique_Change;
    if node_clique.nodes.len() < pred_clique.nodes.len() {
        change = Clique_Change::new(node_index, node_clique.nodes.clone(), is_source);
    } else {
        change = Clique_Change::new(node_index, pred_clique.nodes.clone(), is_source);
    }

    // merge pred_clique into node_clique and empty pred_clique
    // (Vi kan ikke bare fjerne pred_clique, da vores værdier i index_map bliver fucked)
    node_clique.merge(&pred_clique);
    clique[node_index] = node_clique;
    clique[pred_index] = Clique::empty();

    // Update index_map
    index_map::update_index_map(index_map, &pred_clique, node_index, is_source);

    return change;
}
