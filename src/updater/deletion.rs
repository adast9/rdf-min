use std::collections::HashMap;

use crate::parser::{clique::Clique, meta_parser::NodeInfo, triple::Triple, MetaData};

use super::{funcs::{remove_from_supernode, remove_parent, to_single_node, get_node_index}, intersects};

pub fn delete_triple(
    deletions: &Vec<Triple>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    metadata: &mut MetaData,
) {
    for deletion in deletions {
        handle_deletion(metadata, deletion, source_clique, target_clique);
    }
}

fn handle_deletion(
    metadata: &mut MetaData,
    triple: &Triple,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
) {
    // Case_1: if sub has a parent else sub has no parent
    if let Some(_p_sub) = metadata.nodes.get(&triple.sub).unwrap().parent {
        let supernode_id_sub = metadata.supernodes.get(&_p_sub).unwrap().clone()[0];
        handle_parental_nodes(metadata, supernode_id_sub, &_p_sub);
        // If a supernode only consists of a single node, then remove clique
        handle_singleton_supernodes(metadata, source_clique, target_clique, &_p_sub);
    } else {
        handle_non_parental_nodes(metadata, triple, source_clique, true);
    }
    // Case_1: if obj has a parent else obj has no parent
    if let Some(_p_obj) = metadata.nodes.get(&triple.obj).unwrap().parent {
        let supernode_id_obj = metadata.supernodes.get(&_p_obj).unwrap().clone()[0];
        handle_parental_nodes(metadata, supernode_id_obj, &_p_obj);
        // If a supernode only consists of a single node, then remove clique
        handle_singleton_supernodes(metadata, source_clique, target_clique, &_p_obj);
    } else {
        handle_non_parental_nodes(metadata, triple, target_clique, false);
    }
}

fn handle_parental_nodes(
    metadata: &mut MetaData,
    supernode_id: u32,
    nodes_of_parent: &u32,
) {
    // Remove node from a supernode and sets parent to None
    remove_from_supernode(metadata, supernode_id, &nodes_of_parent);
    // Checks for intersections for incoming or outgoing edges
    handle_split(metadata, &nodes_of_parent);
}

fn handle_non_parental_nodes(
    metadata: &mut MetaData,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    is_sub: bool,
) {
    let triple_node = if is_sub { &triple.sub } else { &triple.obj};
    // Remove edges from sub or obj
    remove_edge(&mut metadata.nodes, triple, is_sub);
    // Remove node from clique
    remove_preds_in_clique(metadata, triple, clique, is_sub);
    // Remove if edges of sub or obj are empty
    handle_empty_edge(metadata, &triple_node, clique, is_sub);
    // Group overlapping nodes in clique and remove the single nodes from the clique
    handle_overlaps(metadata, &triple_node, clique);
}

fn handle_overlaps(
    metadata: &mut MetaData,
    node: &u32,
    clique: &mut Vec<Clique>,
) {
    let node_index = get_node_index(metadata, &node, 0);
    let mut new_clique_nodes: Vec<Vec<u32>> = Vec::new();
    new_clique_nodes.push(clique[node_index].nodes.clone());

    let mut nodes = clique[node_index].nodes.clone();

    for node in &mut nodes {
        let mut overlapping: Vec<u32> = Vec::new();
        for (index, new_clique_node) in &mut new_clique_nodes.iter().enumerate() {
            generate_new_clique_node(new_clique_node, node, &mut overlapping, 
                                    metadata, index, &mut new_clique_nodes);
        }
    }
}

fn generate_new_clique_node(
    new_clique_node: &Vec<u32>, 
    node: &mut u32, 
    overlapping: &mut Vec<u32>, 
    metadata: &mut MetaData, 
    index: usize, 
    new_clique_nodes: &mut Vec<Vec<u32>>
) {
    for cnode in new_clique_node {
        if  node == cnode {
            continue;
        }
        *overlapping = handle_case_for_overlaps(node, cnode, metadata, overlapping, index);
    }
    // Group overlapping nodes in new_clique_nodes and remove the single nodes from new_clique_nodes
    merge_nodes_for_clique_overlaps(&*overlapping, new_clique_nodes);
}

// Group overlapping nodes in new_clique_nodes and remove the single nodes from new_clique_nodes
fn merge_nodes_for_clique_overlaps(
    overlapping: &Vec<u32>, 
    new_clique_nodes: &mut Vec<Vec<u32>>
) {
    if overlapping.len() > 0 {
        let mut new_clique_node: Vec<u32> = Vec::new();
        for (j, _o) in overlapping.iter().enumerate() {
            new_clique_node.append(&mut new_clique_nodes[j]);
        }
        new_clique_nodes.push(new_clique_node);
    }
}

// Identifies overlaps in different cases for parental and non-parental nodes
fn handle_case_for_overlaps(
    node: &mut u32, 
    cnode: &u32, 
    metadata: &mut MetaData, 
    overlapping: &mut Vec<u32>, 
    index: usize
) -> Vec<u32> {

    if check_has_parent(node, metadata) && check_has_parent(cnode, metadata) {
        return identify_overlaps_for_parent(metadata, node, cnode, overlapping, index);

    } else if check_has_parent(node, metadata) {
        let cnode_outgoing_preds = get_preds_in_edges(&metadata.nodes.get(cnode).unwrap().outgoing);
        return identify_overlaps(metadata, node, cnode_outgoing_preds, overlapping, index);    

    } else if check_has_parent(cnode, metadata) {
        let node_outgoing_preds = get_preds_in_edges(&metadata.nodes.get(node).unwrap().outgoing);
        return identify_overlaps(metadata, cnode, node_outgoing_preds, overlapping, index);
        
    } else {
        let node_outgoing_preds = get_preds_in_edges(&metadata.nodes.get(node).unwrap().outgoing);
        let cnode_outgoing_preds = get_preds_in_edges(&metadata.nodes.get(&cnode).unwrap().outgoing);

        if intersects(&node_outgoing_preds,&cnode_outgoing_preds) {
            overlapping.push(index as u32);
        }
        return overlapping.clone();
    }
}

// When both nodes have parents, we need to check for overlaps between the parents
fn identify_overlaps_for_parent(
    metadata: &MetaData, 
    node1: &u32, 
    node2: &u32, 
    overlapping: &mut Vec<u32>, 
    index: usize
) -> Vec<u32>{
    let snode = metadata.supernodes.get(&node1).unwrap().clone();
    let cnode = metadata.supernodes.get(&node2).unwrap().clone();

    for n in snode {
        let node_outgoing_preds = get_preds_in_edges(&metadata.nodes.get(&n).unwrap().outgoing);
        for c in &cnode {
            let cnode_outgoing_preds = get_preds_in_edges(&metadata.nodes.get(&c).unwrap().outgoing);
            if intersects(&node_outgoing_preds,&cnode_outgoing_preds) {
                overlapping.push(index as u32);
            }
        }
    }
    return overlapping.clone();
}

// For non-parental, we need to check for overlaps between singleton nodes
fn identify_overlaps(
    metadata: &MetaData, 
    node: &u32, 
    outgoing_preds: Vec<u32>, 
    overlapping: &mut Vec<u32>, 
    index: usize
) -> Vec<u32> {
    let snode = metadata.supernodes.get(&node).unwrap().clone();
    
    let node_outgoing_preds = remove_duplicates(&snode);

    if intersects(&node_outgoing_preds,&outgoing_preds) {
        overlapping.push(index as u32);
    }
    return overlapping.clone();
}

// Remove duplicates from node_outgoing_preds
fn remove_duplicates(snode: &Vec<u32>) -> Vec<u32> {
    let mut node_outgoing_preds_unique: Vec<u32> = Vec::new();
    for p in snode {
        if !node_outgoing_preds_unique.contains(p) {
            node_outgoing_preds_unique.push(*p);
        }
    }
    return node_outgoing_preds_unique;
}

fn check_has_parent(
    node: &u32, 
    metadata: &MetaData
) -> bool {
    if let Some(_p) = metadata.nodes.get(node).unwrap().parent {
        return true;
    }
    return false;
}

/// Removes incoming or outgoing edges from `node`.
fn remove_triple_from_node(
    nodes: &mut HashMap<u32, NodeInfo>, 
    triple: &Triple
) {
    remove_edge(nodes, triple, true);
    remove_edge(nodes, triple, false);
}

/// Removes predicate `pred` from a `clique`.
fn remove_preds_in_clique(
    metadata: &mut MetaData,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    is_sub: bool
) {
    let node = if is_sub { triple.sub } else { triple.obj };

    let node_index = get_node_index(metadata, &node, 0);
    let edges = group_edges(metadata.nodes.get(&triple.sub).unwrap(), is_sub);

    if clique[node_index].nodes.len().clone() == 1 {
        // Check if clique[node_index].preds contains outgoing_edges
        for i in 0..clique[node_index].preds.len() {
            clique[node_index].preds.retain(|x| x != &edges[i]);
        }
    }
}

/// Group all outgoing (true) or incoming (false) edges into one vector
fn group_edges(
    nodeinfo: &NodeInfo, 
    is_outgoing: bool
) -> Vec<u32> {
    let mut edges: Vec<u32> = Vec::new();
    let nodeinfo_edge = if is_outgoing { &nodeinfo.outgoing } else { &nodeinfo.incoming };

    for edge in &mut nodeinfo_edge.clone() {
        edges.append(edge);
    }
    return edges;
}

// CHECK FOR CORRECTNESS
// If edge is empty, then move node in clique to empty clique
fn handle_empty_edge(
    metadata: &mut MetaData,
    node: &u32,
    clique: &mut Vec<Clique>,
    is_outgoing: bool
) {
    let nodeinfo = metadata.nodes.get(node).unwrap();
    if is_edge_empty(nodeinfo, is_outgoing) {
        let sc_index = metadata.index_map.get(node).unwrap()[0];
        clique[0].add_node(&(sc_index as u32));
        clique.remove(sc_index);
    } else {
        let tc_index = metadata.index_map.get(node).unwrap()[1];
        clique[0].add_node(&(tc_index as u32));
        clique.remove(tc_index);
    }
}

// Check if edges is empty
fn is_edge_empty(nodeinfo: &NodeInfo, is_outgoing: bool) -> bool {
    let edges = if is_outgoing { &nodeinfo.outgoing } else { &nodeinfo.incoming };

    if edges.is_empty() {
        return true;
    }
    return false;
}

fn remove_edge(
    nodes: &mut HashMap<u32, NodeInfo>,
    triple: &Triple,
    is_sub: bool,
) {
    let node = if is_sub { triple.sub } else { triple.obj };
    if is_sub {
        nodes.get_mut(&node).unwrap().outgoing.
        retain(|x| x[1] != node && x[0] != triple.pred);
    } else {
        nodes.get_mut(&node).unwrap().incoming.
        retain(|x| x[1] != node && x[0] != triple.pred);
    };
}

/// Handles the case where a supernode only contains one node.
fn handle_singleton_supernodes(
    metadata: &mut MetaData,
    sc: &mut Vec<Clique>,
    tc: &mut Vec<Clique>,
    snode: &u32,
) {
    let source_index = metadata.index_map.get(snode).unwrap()[0];
    let target_index = metadata.index_map.get(snode).unwrap()[1];

    // if the node is the only node in the parent, remove the parent
    if metadata.supernodes.get(snode).unwrap().len() == 1 {
        to_single_node(metadata, sc, tc, &snode, source_index, target_index);
    }
}

/// Handles the split of a supernode.
fn handle_split(
    metadata: &mut MetaData, 
    node: &u32
) {
    let nodeinfo = metadata.nodes.get(node).unwrap();
    let supernode_id = nodeinfo.parent.unwrap();
    let mut snode = metadata.supernodes.get(&supernode_id).unwrap().clone();
    let mut node_out_preds = Vec::new();
    let mut node_inc_preds = Vec::new();

    // Get every pred in `outgoing`
    for edge in &nodeinfo.outgoing.clone() {
        node_out_preds.push(edge[0]);
    }
    // Get every pred in `incoming`
    for edge in &nodeinfo.incoming.clone() {
        node_inc_preds.push(edge[0]);
    }

    snode.retain(|x| x != node);

    let (rest_inc, rest_out) = get_inc_and_out(&metadata.nodes, &snode);

    let rest_out_preds = get_preds_in_edges(&rest_out);
    let rest_inc_preds = get_preds_in_edges(&rest_inc);

    // Checks if there is an intersection between the incoming of the pivotal node and the rest of the supernode
    // It checks also for the outgoing case in similar fashion.
    if intersects(&node_inc_preds, &rest_inc_preds) || intersects(&node_out_preds, &rest_out_preds)
    {
        // if there is no intersection, remove the supernode
        remove_from_supernode(metadata, supernode_id, node);
    }
}

/// Returns a tuple of incoming and outgoing edges for a node.
fn get_inc_and_out(
    nodes: &HashMap<u32, NodeInfo>,
    snode: &Vec<u32>,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut inc: Vec<Vec<u32>> = Vec::new();
    let mut out: Vec<Vec<u32>> = Vec::new();

    for n in snode {
        let nodeinfo = nodes.get(n).unwrap();
        inc.append(&mut nodeinfo.incoming.clone());
        out.append(&mut nodeinfo.outgoing.clone());
    }
    return (inc, out);
}

// Get each pred for each edge
fn get_preds_in_edges(
    edges: &Vec<Vec<u32>>
) -> Vec<u32> {
    let mut preds: Vec<u32> = Vec::new();
    for edge in edges {
        if !preds.contains(&edge[0]){
            preds.push(edge[0]);
        }
    }
    return preds;
}

/// Returns `true` if there is an intersection between two vectors of vectors of u32s else `false`.
fn intersects_for_vec_vec(
    v1: &Vec<Vec<u32>>, 
    v2: &Vec<Vec<u32>>
) -> bool {
    for n in v1 {
        if v2.contains(&n) {
            return true;
        }
    }
    return false;
}