use std::collections::HashMap;

use crate::parser::{clique::Clique, meta_parser::NodeInfo, triple::Triple, MetaData};

use super::{funcs::{remove_from_supernode, to_single_node, get_node_index}, intersects};

pub fn delete_triple(
    deletions: &Vec<Triple>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    metadata: &mut MetaData
) {
    for deletion in deletions {
        handle_deletion(metadata, deletion, source_clique, target_clique);
    }
}

// Handles deletion of subject node and object node in a triple
fn handle_deletion(
    metadata: &mut MetaData,
    triple: &Triple,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
) {
    // Case_1: if sub has a parent else sub has no parent
    if let Some(_p_sub) = metadata.nodes.get(&triple.sub).unwrap().parent {
        // Remove edge from node
        remove_edge(&mut metadata.nodes, triple, true);
        // Handle a split of a supernode if no intersection has been found between nodes in snode
        handle_split(metadata, &_p_sub);
        // If a supernode only consists of a single node, then remove clique
        handle_singleton_supernodes(metadata, source_clique, target_clique, &_p_sub);
    } else {
        handle_nodes(metadata, triple, source_clique, true);
    }
    // Case_1: if obj has a parent else obj has no parent
    if let Some(_p_obj) = metadata.nodes.get(&triple.obj).unwrap().parent {
        // Remove edge from node
        remove_edge(&mut metadata.nodes, triple, false);
        // Handle a split of a supernode if no intersection has been found between nodes in snode
        handle_split(metadata, &_p_obj);
        // If a supernode only consists of a single node, then remove clique
        handle_singleton_supernodes(metadata, source_clique, target_clique, &_p_obj);
    } else {
        handle_nodes(metadata, triple, target_clique, false);
    }
}

/// Handles the case where a supernode only contains one node.
fn handle_singleton_supernodes(
    metadata: &mut MetaData,
    sc: &mut Vec<Clique>,
    tc: &mut Vec<Clique>,
    snode: &u32
) {
    let source_index = metadata.index_map.get(snode).unwrap()[0];
    let target_index = metadata.index_map.get(snode).unwrap()[1];

    // if the node is the only node in the parent, remove the parent and set the parent of the node to None
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

    // Get every unique pred in `outgoing` respectively to `node`
    for edge in &nodeinfo.outgoing.clone() {
        node_out_preds.push(edge[0]);
    }

    // Get every unique pred in `incoming` respectively to `node`
    for edge in &nodeinfo.incoming.clone() {
        node_inc_preds.push(edge[0]);
    }

    // Removes the node we have to compare from snode, 
    // such that we don't compare its inc and out with itself later
    snode.retain(|x| x != node);

    // Gets all inc and out preds of the nodes in snode. Node is not included.
    let (rest_inc, rest_out) = get_inc_and_out(&metadata.nodes, &snode);

    let rest_out_preds = get_preds_in_edges(&rest_out);
    let rest_inc_preds = get_preds_in_edges(&rest_inc);

    // if there is no intersection, remove the supernode
    if !intersects(&node_inc_preds, &rest_inc_preds) || intersects(&node_out_preds, &rest_out_preds)
    {
        remove_from_supernode(metadata, supernode_id, node);
    }
}

/*
// Removes node from the supernode supernode_id and sets its parent to None.
fn handle_supernode(
    metadata: &mut MetaData,
    nodes_of_parent: &u32,
) {

    // Remove node from a supernode and sets parent to None
    remove_from_supernode(metadata, supernode_id, &nodes_of_parent); 

    // Checks for intersections for incoming or outgoing edges
    handle_split(metadata, &nodes_of_parent);
}*/

/// Remove edge from node and checks if the node is a supernode.
fn handle_nodes(
    metadata: &mut MetaData,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    is_sub: bool
) {
    let triple_node = if is_sub { &triple.sub } else { &triple.obj};
    // Remove edges from sub or obj
    remove_edge(&mut metadata.nodes, triple, is_sub);
    // Remove node from clique
    remove_preds_in_clique(metadata, triple, clique, is_sub);
    // Remove if edges of sub or obj are empty
    handle_empty_edge(metadata, &triple_node, clique, is_sub);
    // Group intersected nodes into a clique and remove the original clique
    check_for_new_cliques(metadata, &triple_node, clique, is_sub);
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
    if is_edge_empty(nodeinfo, is_outgoing == true) {
        let sc_index = metadata.index_map.get(node).unwrap()[0];
        clique.remove(sc_index);
        clique[0].add_node(&(sc_index as u32));
    } else if is_edge_empty(nodeinfo, is_outgoing == false) {
        let tc_index = metadata.index_map.get(node).unwrap()[1].clone();
        clique.remove(tc_index);
        clique[0].add_node(&(tc_index as u32));
    }
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

fn check_for_new_cliques(
    metadata: &mut MetaData,
    node: &u32,
    clique: &mut Vec<Clique>,
    is_sub: bool
) -> Vec<Vec<u32>> {
    let node_index = get_node_index(metadata, &node, 0);
    let mut nodes1: Vec<Vec<u32>> = Vec::new();
    let mut nodes2 = clique[node_index].nodes.clone();
    let mut new_clique_nodes: Vec<Vec<u32>> = Vec::new();
    nodes1.push(clique[node_index].nodes.clone());
    let mut resulting_cliques: Vec<Vec<u32>> = Vec::new();

    for n1 in &mut nodes1{
        for n2 in &mut nodes2 {
            for n in &mut *n1 {
                if n == n2 {
                    continue;
                }
                if check_has_parent(&n, metadata) {
                    let snode = metadata.supernodes.get(&n).unwrap().clone();
    
                    for sn in snode {
                        let node_edge_preds1 = get_edge_preds(metadata, &sn, is_sub);
                        let node_edge_preds2 = get_edge_preds(metadata, &n2, is_sub);

                        if intersects(&node_edge_preds1,&node_edge_preds2) {            
                            let mut new_vec_clique: Vec<u32> = Vec::new();
                            new_vec_clique.push(*n);
                            new_vec_clique.push(*n2);
                            new_clique_nodes.push(new_vec_clique.clone());
                            break;
                        } else {
                            // check if a node is contained in a vector in resulting_cliques
                            for v in &mut resulting_cliques {
                                if v.contains(&n) {
                                    new_clique_nodes.push(vec![*n2]);
                                    break;
                                } else {
                                    new_clique_nodes.push(vec![*n2]);
                                    new_clique_nodes.push(vec![*n]);
                                }
                            }
                        }
                    }
                } else if check_has_parent(&n2, metadata) {
                    let snode = metadata.supernodes.get(&n2).unwrap().clone();
    
                    for sn in snode {
                        let node_edge_preds1 = get_edge_preds(metadata, &n, is_sub);
                        let node_edge_preds2 = get_edge_preds(metadata, &sn, is_sub);

                        if intersects(&node_edge_preds1,&node_edge_preds2) {
                            let mut new_vec_clique: Vec<u32> = Vec::new();
                            new_vec_clique.push(*n2);
                            new_vec_clique.push(*n);
                            new_clique_nodes.push(new_vec_clique.clone());
                            break;
                        } else {
                            // check if a node is contained in a vector in resulting_cliques
                            for v in &mut resulting_cliques {
                                if v.contains(&n2) {
                                    new_clique_nodes.push(vec![*n]);
                                    break;
                                } else {
                                    new_clique_nodes.push(vec![*n]);
                                    new_clique_nodes.push(vec![*n2]);
                                }
                            }
                        }
                    }
                } else {
                    let node_edge_preds1 = get_edge_preds(metadata, &n, is_sub);
                    let node_edge_preds2 = get_edge_preds(metadata, &n2, is_sub);

                    if intersects(&node_edge_preds1,&node_edge_preds2) {
                        let mut new_vec_clique: Vec<u32> = Vec::new();
                        new_vec_clique.push(*n);
                        new_vec_clique.push(*n2);
                        new_clique_nodes.push(new_vec_clique.clone());
                    } else {
                        // check if a node is contained in a vector in resulting_cliques
                        for v in &mut resulting_cliques {
                            if v.contains(&n) {
                                new_clique_nodes.push(vec![*n2]);
                                break;
                            } else {
                                new_clique_nodes.push(vec![*n2]);
                                new_clique_nodes.push(vec![*n]);
                            }
                        }
                    }
                }
                resulting_cliques.append(&mut new_clique_nodes);
            }
        }
    }
    sort(&mut resulting_cliques);
    remove_duplicates(&mut resulting_cliques);
    union_cliques(&mut resulting_cliques);
    clear_clique(clique, node_index);
    create_cliques(metadata, &resulting_cliques, clique, is_sub);
    return resulting_cliques;
}

fn clear_clique(clique: &mut Vec<Clique>, node_index: usize) {
    clique[node_index].nodes.clear();
    clique[node_index].preds.clear();
    if !clique.is_empty() {
        panic!();
    }
}

// create new cliques from resulting_cliques
fn create_cliques(
    metadata: &mut MetaData,
    resulting_cliques: &Vec<Vec<u32>>,
    clique: &mut Vec<Clique>,
    is_sub: bool
) {
    for c in resulting_cliques {
        let new_clique = Clique {
            nodes: c.clone(),
            preds: get_edge_preds(metadata, &c[0], is_sub),
        };
        clique.push(new_clique);
    }
}   

// remove duplicates from vector of vectors of u32
fn remove_duplicates(cliques: &mut Vec<Vec<u32>>) {
    let mut new_cliques: Vec<Vec<u32>> = Vec::new();
    for (i,c) in &mut cliques.iter().enumerate() {
        if !new_cliques.contains(&cliques[i]) {
            new_cliques.push(c.clone());
        }
    }
    *cliques = new_cliques.clone();
}

/// Removes predicate `pred` from a `clique`.
fn remove_preds_in_clique(
    metadata: &mut MetaData,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    is_sub: bool
) {
    let node = if is_sub { triple.sub } else { triple.obj };
    let sc_or_tc = if is_sub { 0 } else { 1 };
    let node_index = get_node_index(metadata, &node, sc_or_tc);
    let mut edges = get_all_edges(metadata.nodes.get(&node).unwrap(), is_sub);

    edges.sort();
    edges.dedup();

    // if the clique only contains that one node, then delete all preds in the clique that are not in the edges
    if clique[node_index].nodes.len().clone() == 1 {
        let mut new_preds: Vec<u32> = Vec::new();
        for p in &clique[node_index].preds {
            if edges.contains(&p) {
                new_preds.push(*p);
            }
        }
        if new_preds.is_empty() {
            // remove pred in the other clique
            let sc_or_tc = if !is_sub { 0 } else { 1 };
            let node_index = get_node_index(metadata, &node, sc_or_tc);
            clique[node_index].nodes.remove(node as usize);
            clique[node_index].preds.remove(triple.pred as usize);

            if clique[node_index].nodes.is_empty() {
                clique.remove(node_index);
            }
        }
        clique[node_index].preds = new_preds;
    } 
}

/// Removes an edge from a node in `nodes`.
fn remove_edge(
    nodes: &mut HashMap<u32, NodeInfo>,
    triple: &Triple,
    is_sub: bool
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

/// Returns a tuple of incoming and outgoing edges for a node.
fn get_inc_and_out(
    nodes: &HashMap<u32, NodeInfo>,
    snode: &Vec<u32>
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

/// Get each pred for each edge
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

/// Gets all predicates in an edge for a node.
fn get_edge_preds(
    metadata: &MetaData, 
    node: &u32, 
    is_sub: bool
) -> Vec<u32> {
    let edge_preds = if is_sub { get_preds_in_edges(&metadata.nodes.get(node).unwrap().outgoing) } 
                            else { get_preds_in_edges(&metadata.nodes.get(node).unwrap().incoming) };
    return edge_preds;
}

/// Group all outgoing (true) or incoming (false) edges into one vector
fn get_all_edges(
    nodeinfo: &NodeInfo, 
    is_sub: bool
) -> Vec<u32> {
    let mut edges: Vec<u32> = Vec::new();
    let nodeinfo_edge = if is_sub { &nodeinfo.outgoing } else { &nodeinfo.incoming };

    for edge in &mut nodeinfo_edge.clone() {
        edges.append(edge);
    }
    return edges;
}

/// Checks if edge is empty
fn is_edge_empty(nodeinfo: &NodeInfo, is_outgoing: bool) -> bool {
    let edges = if is_outgoing { &nodeinfo.outgoing } else { &nodeinfo.incoming };
    if edges.is_empty() {
        return true;
    }
    return false;
}

//[[A,b], [b,c], [c], [b,A]]
/// Merge two vectors of `u32` if they have a intersection
fn union_cliques(cliques: &mut Vec<Vec<u32>>) {
    let mut new_cliques: Vec<Vec<u32>> = Vec::new();
    for (i,c) in &mut cliques.iter().enumerate() {
        for (j,c2) in &mut cliques.iter().enumerate() {
            if i != j {
                if intersects(&c,&c2) {
                    let mut new_vec: Vec<u32> = Vec::new();
                    new_vec.append(&mut c.clone());
                    new_vec.append(&mut c2.clone());
                    new_cliques.push(new_vec.clone());
                } else {
                    new_cliques.push(c.clone());
                    new_cliques.push(c2.clone());
                }
            }
        }
    }
    *cliques = new_cliques.clone();
}

/// Sort elements in vectors of vectors
fn sort(vec: &mut Vec<Vec<u32>>) {
    for v in vec {
        v.sort();
    }
}