use crate::{
    parser::{clique::Clique, triple::Triple, MetaData},
    updater::funcs::{
        get_name, get_node_index, index_of_empty_clique, merge_cliques, node_is_in_supernode,
        node_is_supernode, remove_from_supernode, split_snode_name, to_single_node,
        update_clique_indices, update_index, update_triples_after_split,
    },
};

use super::CliqueChange;

pub fn insert(
    stuff: &mut MetaData,
    triple: &Triple,
    clique: &mut Vec<Clique>,
    other_clique: &mut Vec<Clique>,
    is_source: bool,
) -> Option<CliqueChange> {
    let i = if is_source { 0 } else { 1 };
    let node = if is_source { &triple.sub } else { &triple.obj };

    // Get indices of cliques containing pred and node
    let pred_index = stuff.index_map.get(&triple.pred).unwrap()[i];
    let node_index = get_node_index(stuff, node, i);

    // CASE 1: If node and pred are in the same clique, return None
    if node_index == pred_index {
        return None;
    }

    // CASE 2: If node is not in the empty set clique, merge cliques
    if node_index != index_of_empty_clique(clique) {
        let change = CliqueChange::new(
            pred_index,
            if clique[node_index].nodes.len() < clique[pred_index].nodes.len() {
                clique[node_index].nodes.clone()
            } else {
                clique[pred_index].nodes.clone()
            },
            is_source,
        );

        merge_cliques(clique, pred_index, node_index);
        update_clique_indices(&mut stuff.index_map, &clique[pred_index], pred_index, i);

        return Some(change);
    }

    // CASE 3: If node is not a supernode, but in the empty set clique, move node to pred clique
    if !node_is_in_supernode(node, &mut stuff.supernodes) {
        clique[pred_index].add_node(node);
        clique[node_index].remove_node(node);

        update_index(&mut stuff.index_map, node, pred_index, i);

        return Some(CliqueChange::new(pred_index, vec![*node], is_source));
    }

    // CASE 4: If node is a supernode AND in the empty set clique, split node from its supernode
    // todo: remove the split node's name from the supernode
    let snode = stuff.nodes.get(node).unwrap().parent.unwrap();

    remove_from_supernode(stuff, snode, node);
    clique[pred_index].add_node(node);

    let other_clique_index = stuff.index_map.get(&snode).unwrap()[if is_source { 1 } else { 0 }];
    other_clique[other_clique_index].add_node(node);

    stuff
        .index_map
        .insert(*node, [pred_index, other_clique_index]);

    let new_triples = update_triples_after_split(stuff, node, &snode);
    if let Some(new_triples) = new_triples {
        stuff.triples.extend(new_triples);
    }

    if stuff.supernodes.get(&snode).unwrap().len() == 1 {
        to_single_node(
            stuff,
            clique,
            other_clique,
            &snode,
            node_index,
            other_clique_index,
        );
    } else {
        split_snode_name(&mut stuff.dict, &snode, node);
    }

    return Some(CliqueChange::new(pred_index, vec![*node], is_source));
}
