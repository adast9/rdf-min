use std::collections::HashMap;

use crate::parser::{clique::Clique, meta_parser::NodeInfo, triple::Triple, MetaData};

/// Gets all incoming and outgoing edges for all nodes in `ids`.
pub fn get_edges(nodes: &HashMap<u32, NodeInfo>, ids: &Vec<u32>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut incoming: Vec<Vec<u32>> = Vec::new();
    let mut outgoing: Vec<Vec<u32>> = Vec::new();

    for id in ids {
        let node = nodes.get(id).unwrap();

        for i in &node.incoming {
            let mut x = i.clone();
            if nodes.contains_key(&x[1]) {
                if let Some(p) = nodes.get(&x[1]).unwrap().parent {
                    x[1] = p;
                }
            }
            incoming.push(x);
        }
        for o in &node.outgoing {
            let mut x = o.clone();
            if nodes.contains_key(&x[1]) {
                if let Some(p) = nodes.get(&x[1]).unwrap().parent {
                    x[1] = p;
                }
            }
            outgoing.push(x);
        }
    }

    return (incoming, outgoing);
}

/// Removes `node` from the supernode `supernode_id` and sets its parent to `None`.
pub fn remove_from_supernode(stuff: &mut MetaData, supernode_id: u32, node: &u32) {
    stuff
        .supernodes
        .get_mut(&supernode_id)
        .unwrap()
        .retain(|x| *x != *node);
    remove_parent(&mut stuff.nodes, node);
}

/// Sets the parent of `node` to `None`.
pub fn remove_parent(nodes: &mut HashMap<u32, NodeInfo>, node: &u32) {
    nodes.get_mut(node).unwrap().parent = None;
}

/// Sets the parent of `node` to `new_parent`.
pub fn new_parent(nodes: &mut HashMap<u32, NodeInfo>, node: &u32, new_parent: &u32) {
    nodes.get_mut(node).unwrap().parent = Some(*new_parent);
}

/// Gets the index of the clique containing `node`.
///
/// `i = 0` for source clique, `i = 1` for target clique.
pub fn get_node_index(stuff: &mut MetaData, node: &u32, i: usize) -> usize {
    if let Some(p) = stuff.nodes.get(node).unwrap().parent {
        return stuff.index_map.get(&p).unwrap()[i];
    } else {
        return stuff.index_map.get(node).unwrap()[i];
    }
}

/// Return true if `node` is a supernode. Else false
pub fn node_is_supernode(node: &u32, supernodes: &mut HashMap<u32, Vec<u32>>) -> bool {
    if supernodes.contains_key(node) {
        return true;
    }
    return false;
}

pub fn node_is_in_supernode(node: &u32, supernodes: &mut HashMap<u32, Vec<u32>>) -> bool {
    for sn in supernodes.values() {
        if sn.contains(node) {
            return true;
        }
    }
    return false;
}

/// Merges the contents of two cliques by their indices.
///
/// Merges into the `i1` clique and leaves `i2` empty.
pub fn merge_cliques(clique: &mut Vec<Clique>, i1: usize, i2: usize) {
    let n = clique[i2].nodes.clone();
    let p = clique[i2].preds.clone();

    clique[i1].nodes.extend(n);
    clique[i1].preds.extend(p);

    clique[i2] = Clique::empty();
}

/// Updates the indices of all nodes and predicates in `clique` to `new_index`.
///
/// `i = 0` for source clique, `i = 1` for target clique.
pub fn update_clique_indices(
    index_map: &mut HashMap<u32, [usize; 2]>,
    clique: &Clique,
    new_index: usize,
    i: usize,
) {
    for node in &clique.nodes {
        let arr = index_map.get_mut(node).unwrap();
        arr[i] = new_index;
    }
    for pred in &clique.preds {
        let arr = index_map.get_mut(pred).unwrap();
        arr[i] = new_index;
    }
}

/// Updates the index of `node` to `new_index`.
pub fn update_index(
    index_map: &mut HashMap<u32, [usize; 2]>,
    node: &u32,
    new_index: usize,
    i: usize,
) {
    let arr = index_map.get_mut(node).unwrap();
    arr[i] = new_index;
}

/// Updates all `triples` after `node` has been split from `snode`.
///
/// If the update requires new triples to be made, they are returned.
pub fn update_triples_after_split(
    stuff: &mut MetaData,
    node: &u32,
    snode: &u32,
) -> Option<Vec<Triple>> {
    let mut new_triples: Vec<Triple> = Vec::new();

    for triple in &mut stuff.triples {
        let new_triple =
            update_triple_after_split(&stuff.nodes, &stuff.supernodes, triple, node, snode);

        if let Some(new_triple) = new_triple {
            new_triples.push(new_triple);
        }
    }

    if new_triples.len() > 0 {
        return Some(new_triples);
    } else {
        return None;
    }
}

/// Updates the `triple` after `node` has been split from `snode`.
///
/// If the update requires a new triple to be made, it is returned.
fn update_triple_after_split(
    nodes: &HashMap<u32, NodeInfo>,
    supernodes: &HashMap<u32, Vec<u32>>,
    triple: &mut Triple,
    node: &u32,
    snode: &u32,
) -> Option<Triple> {
    // todo: Generalize this

    // When the supernode is the subject
    if triple.sub == *snode {
        let (_single_inc, single_out) = get_edges(&nodes, &vec![*node]);

        if !single_out.contains(&vec![triple.pred, triple.obj]) {
            return None;
        }

        let (_super_inc, super_out) = get_edges(&nodes, supernodes.get(snode).unwrap());

        if super_out.contains(&vec![triple.pred, triple.obj]) {
            Some(Triple {
                sub: *node,
                pred: triple.pred,
                obj: triple.obj,
                is_type: triple.is_type,
            })
        } else {
            triple.sub = *node;
            None
        }
    }
    // When the supernode is the object
    else if triple.obj == *snode {
        let (single_inc, _single_out) = get_edges(nodes, &vec![*node]);

        if !single_inc.contains(&vec![triple.pred, triple.sub]) {
            return None;
        }

        let (super_inc, _super_out) = get_edges(nodes, supernodes.get(snode).unwrap());

        if super_inc.contains(&vec![triple.pred, triple.sub]) {
            Some(Triple {
                sub: triple.sub,
                pred: triple.pred,
                obj: *node,
                is_type: triple.is_type,
            })
        } else {
            triple.obj = *node;
            None
        }
    } else {
        None
    }
}

/// Turns a supernode with 1 element `snode` into a single node.
pub fn to_single_node(
    stuff: &mut MetaData,
    clique: &mut Vec<Clique>,
    other_clique: &mut Vec<Clique>,
    snode: &u32,
    node_index: usize,
    other_clique_index: usize,
) {
    let node = stuff.supernodes.get(&snode).unwrap()[0];

    stuff.supernodes.remove(&snode);
    remove_parent(&mut stuff.nodes, &node);

    clique[node_index].remove_node(snode);
    clique[node_index].add_node(&node);
    other_clique[other_clique_index].remove_node(snode);
    other_clique[other_clique_index].add_node(&node);

    replace_node_index(&mut stuff.index_map, snode, &node);
    replace_all_triple(&mut stuff.triples, snode, &node)
}

/// Replaces the `old` node entrance with `new` in `index_map`.
fn replace_node_index(index_map: &mut HashMap<u32, [usize; 2]>, old: &u32, new: &u32) {
    let val = index_map.get(old).unwrap().clone();
    index_map.remove(old);
    index_map.insert(*new, val);
}

/// Replaces all occurrences of the sub/obj `old` with `new` in `triples`.
fn replace_all_triple(triples: &mut Vec<Triple>, old: &u32, new: &u32) {
    for triple in triples {
        if triple.sub == *old {
            triple.sub = *new;
        }
        if triple.obj == *old {
            triple.obj = *new;
        }
    }
}

pub fn index_of_empty_clique(cliques: &Vec<Clique>) -> usize {
    for (i, clique) in cliques.iter().enumerate() {
        if clique.preds.is_empty() && !clique.nodes.is_empty() {
            return i;
        }
    }
    panic!("Trouble finding the empty-set clique. This might be able to happen in rare cases. Ask Esben");
}

pub fn add_unknown_node_and_pred_to_clique(
    node: u32,
    pred: u32,
    cliques: &mut Vec<Clique>,
    other_cliques: &mut Vec<Clique>,
) {
    cliques.push(Clique::new(&vec![pred], &vec![node]));
    let empty_index = index_of_empty_clique(&other_cliques);
    other_cliques[empty_index].add_node(&node);
}

pub fn add_id_to_index_map(
    index_map: &mut HashMap<u32, [usize; 2]>,
    id: &u32,
    sc: &Vec<Clique>,
    tc: &Vec<Clique>,
) {
    let sc_index = index_of_id_in_cliques(id, sc);
    let tc_index = index_of_id_in_cliques(id, tc);
    index_map.insert(*id, [sc_index, tc_index]);
}

fn index_of_id_in_cliques(id: &u32, cliques: &Vec<Clique>) -> usize {
    for (i, clique) in cliques.iter().enumerate() {
        if clique.nodes.contains(id) || clique.preds.contains(id) {
            return i;
        }
    }
    panic!("Node not found in cliques!");
}

pub fn get_key_by_value(dict: &HashMap<String, u32>, value: &u32) -> String {
    for (key, val) in dict {
        if val == value {
            return key.clone();
        }
    }
    panic!("Value not found in dict!");
}

pub fn add_unknown_pred_to_clique(
    stuff: &mut MetaData,
    clique: &mut Vec<Clique>,
    pred: &u32,
    node: &u32,
    i: usize,
) -> usize {
    let node_index = get_node_index(stuff, node, i);

    if !clique[node_index].preds.is_empty() {
        clique[node_index].preds.push(*pred);
        return node_index;
    }

    clique[node_index].remove_node(node);
    clique.push(Clique::new(&vec![*pred], &vec![*node]));
    return clique.len() - 1;
}
