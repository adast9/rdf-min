use std::collections::HashMap;

use crate::parser::{clique::Clique, index_map, meta_parser::NodeInfo, triple::Triple};

use self::clique_updater::Clique_Change;

mod clique_updater;

pub fn run(
    dict: &mut HashMap<String, u32>,
    mut triples: Vec<Triple>,
    insertions: Vec<Triple>,
    deletions: Vec<Triple>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    mut index_map: HashMap<u32, [usize; 2]>,
    mut supernodes: HashMap<u32, Vec<u32>>,
    mut nodes: HashMap<u32, NodeInfo>,
) {
    handle_insersertions(
        insertions,
        &mut index_map,
        source_clique,
        target_clique,
        &mut triples,
        &mut supernodes,
        &mut nodes,
    );
}

fn handle_insersertions(
    insertions: Vec<Triple>,
    index_map: &mut HashMap<u32, [usize; 2]>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    triples: &mut Vec<Triple>,
    supernodes: &mut HashMap<u32, Vec<u32>>,
    nodes: &mut HashMap<u32, NodeInfo>,
) {
    for ins in insertions {
        let changes = clique_updater::get_changes(index_map, &ins, source_clique, target_clique);
        let snodes = get_super_nodes(changes, source_clique, target_clique, index_map);
    }
}

fn get_super_nodes(
    changes: Vec<Clique_Change>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    index_map: &mut HashMap<u32, [usize; 2]>, // triples: &mut Vec<Triple>,
                                              // supernodes: &mut HashMap<u32, Vec<u32>>,
                                              // nodes: &mut HashMap<u32, NodeInfo>
) {
    // for each change
    for cc in changes {
        if cc.is_source {
            let sc = source_clique[cc.clique_index].clone();

            let mut super_nodes: Vec<Vec<u32>> = Vec::new();

            for nn in cc.new_nodes {
                let index = index_map.get(&nn).unwrap()[1];
                let tc = target_clique[index].clone();

                let intersect = sc.node_intersection(&tc);
                if intersect.len() > 0 {
                    super_nodes.push(intersect);
                }
            }


        }
    }
}

// fn handle_clique_change() {
//     unimplemented!();
// }
