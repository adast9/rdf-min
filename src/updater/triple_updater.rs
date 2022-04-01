use std::collections::HashMap;

use crate::parser::{clique::Clique, triple::Triple, MetaData};

use super::funcs::{get_name, new_parent};

pub fn update_changes(
    stuff: &mut MetaData,
    snodes: &Vec<Vec<u32>>,
    sc: &mut Vec<Clique>,
    tc: &mut Vec<Clique>,
) {
    for snode in snodes {
        let new_node = dict_new_snode(stuff, snode);
        update_triples(&mut stuff.triples, snode, &new_node);
        update_index_map(&mut stuff.index_map, snode, &new_node);
        update_supernodes(stuff, snode, &new_node);
        update_cliques(stuff, sc, tc, snode, &new_node);
    }
}

fn update_cliques(
    stuff: &mut MetaData,
    sc: &mut Vec<Clique>,
    tc: &mut Vec<Clique>,
    snode: &Vec<u32>,
    new_node: &u32,
) {
    let sc_index = stuff.index_map.get(new_node).unwrap()[0];
    let tc_index = stuff.index_map.get(new_node).unwrap()[1];

    for node in snode {
        sc[sc_index].nodes.retain(|x| *x != *node);
        tc[tc_index].nodes.retain(|x| *x != *node);
    }

    sc[sc_index].nodes.push(*new_node);
    tc[tc_index].nodes.push(*new_node);
}

/// Combines all nodes in `snode` into a single supernode in `stuff.supernodes`.
/// Also updates the `parent` field of all nodes in `snode`.
fn update_supernodes(stuff: &mut MetaData, snode: &Vec<u32>, new_node: &u32) {
    let mut singlenodes_in_new_supernode: Vec<u32> = Vec::new();

    for node in snode {
        if stuff.supernodes.contains_key(node) {
            let supernode = stuff.supernodes.get(node).unwrap();
            singlenodes_in_new_supernode.extend(supernode);

            for singlenode in supernode {
                new_parent(&mut stuff.nodes, singlenode, &new_node);
            }

            stuff.supernodes.remove(node);
        } else {
            singlenodes_in_new_supernode.push(*node);
            new_parent(&mut stuff.nodes, node, &new_node);
        }
    }

    stuff
        .supernodes
        .insert(*new_node, singlenodes_in_new_supernode);
}

/// Removes all nodes in `snode` and inserts `new_node`.
fn dict_new_snode(stuff: &mut MetaData, snode: &Vec<u32>) -> u32 {
    let mut snode_string = stuff.dict.key_by_value(&snode[0]).unwrap();
    if stuff.supernodes.contains_key(&snode[0]) {
        stuff.dict.remove2(&snode_string);
    }

    snode_string = remove_angle_bracket_at_end(&snode_string).to_string();

    for node in snode.iter().skip(1) {
        let node_string = stuff.dict.key_by_value(node).unwrap();
        snode_string.push_str("_");
        snode_string.push_str(&get_name(&node_string));
        if stuff.supernodes.contains_key(node) {
            stuff.dict.remove2(&node_string);
        }
    }

    snode_string.push_str(">");
    return stuff.dict.add2(&snode_string);
}

fn remove_angle_bracket_at_end(string: &String) -> &str {
    let mut chars = string.chars();
    chars.next_back();

    return chars.as_str();
}

/// Replaces all occurences of a node in `snode`  with `new_node` in `triples`.
fn update_triples(triples: &mut Vec<Triple>, snode: &Vec<u32>, new_node: &u32) {
    for triple in triples {
        for node in snode {
            if &triple.obj == node {
                triple.obj = *new_node;
            }
            if &triple.sub == node {
                triple.sub = *new_node;
            }
        }
    }
}

/// Deletes all occurences of a node in `snode`  in `index_map`. `new_node` is inserted instead.
fn update_index_map(index_map: &mut HashMap<u32, [usize; 2]>, snode: &Vec<u32>, new_node: &u32) {
    let val = index_map.get(&snode[0]).unwrap().clone();

    for node in snode {
        index_map.remove(node);
    }

    index_map.insert(*new_node, val);
}
