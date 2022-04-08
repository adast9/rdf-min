use crate::classes::{clique::CliqueCollection, dataset::Dataset, meta::Meta};

use self::{insertions::CliqueChange, triple_updater::update_changes};
pub mod funcs;
mod insertions;
mod triple_updater;

pub fn run(
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) {
    handle_insersertions(dataset, meta, sc, tc);
}

fn handle_insersertions(
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) {
    for ins in dataset.insertions.data_triples {
        let changes = insertions::get_changes(&ins, dataset, meta, sc, tc);

        if changes.is_empty() {
            continue;
        }

        let snodes = get_super_nodes(stuff, changes, sc, tc);
        update_changes(stuff, &snodes, sc, tc);
    }
}

pub fn get_super_nodes(
    changes: Vec<CliqueChange>,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<Vec<u32>> {
    if changes.len() == 1 {
        return handle_clique_change(stuff, changes[0].clone(), sc, tc);
    }

    let mut cc1 = handle_clique_change(stuff, changes[0].clone(), sc, tc);
    let cc2 = handle_clique_change(stuff, changes[1].clone(), sc, tc);

    let mut done: Vec<Vec<u32>> = Vec::new();
    let mut marks: Vec<[usize; 2]> = Vec::new();

    // todo: Fix dirty clone - make in-place
    for (i, sn1) in cc1.clone().iter().enumerate() {
        let mut used = false;
        for (j, sn2) in cc2.iter().enumerate() {
            if intersects(sn1, sn2) {
                marks.push([i, j]);
                cc1[i] = union(&sn1, sn2);
                used = true;
            }
        }
        if !used {
            done.push(sn1.clone());
        }
    }

    // merge marked supernodes in cc1
    for (i, m) in marks.iter().enumerate() {
        for j in 0..i {
            if m[1] == marks[j][1] {
                let target_i = marks[j][0];
                cc1[target_i] = union(&cc1[target_i], &cc1[m[0]]);
                break;
            }
        }
    }

    // Get done supernodes from cc1
    let mut used: Vec<usize> = Vec::new();
    for m in marks {
        if !used.contains(&m[1]) {
            used.push(m[1]);
            done.push(cc1[m[0]].clone());
        }
    }

    // Get unmarked nodes from cc2
    for (i, sn) in cc2.iter().enumerate() {
        if !used.contains(&i) {
            done.push(sn.clone());
        }
    }

    return done;
}

fn handle_clique_change(
    change: CliqueChange,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<Vec<u32>> {
    let mut super_nodes: Vec<Vec<u32>> = Vec::new();

    let c1 = if change.is_source {
        sc[change.clique_index].clone()
    } else {
        tc[change.clique_index].clone()
    };

    for node in change.new_nodes {
        let c2 = if change.is_source {
            let index = stuff.index_map.get(&node).unwrap()[1];
            tc[index].clone()
        } else {
            let index = stuff.index_map.get(&node).unwrap()[0];
            sc[index].clone()
        };

        let intersect = c1.node_intersection(&c2);
        if intersect.len() >= 2 {
            super_nodes.push(intersect);
        }
    }
    return super_nodes;
}

fn intersects(v1: &Vec<u32>, v2: &Vec<u32>) -> bool {
    for n in v1 {
        if v2.contains(&n) {
            return true;
        }
    }
    return false;
}

fn do_intersection(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let mut intersection: Vec<u32> = Vec::new();
    for n in v1 {
        if v2.contains(&n) {
            intersection.push(*n);
        }
    }
    return intersection;
}

fn union(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = v1.clone();

    for e in v2 {
        if !result.contains(e) {
            result.push(*e);
        }
    }

    return result;
}
