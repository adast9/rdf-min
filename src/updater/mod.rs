use crate::classes::{
    clique::CliqueChange, clique::CliqueCollection, dataset::Dataset, meta::Meta,
};

use self::triple_updater::update_changes;
pub mod funcs;
mod insertions;
mod triple_updater;

pub fn run(
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

        let snodes = get_super_nodes(changes, sc, tc);
        update_changes(stuff, &snodes, sc, tc);
    }
}

pub fn get_super_nodes(
    changes: Vec<CliqueChange>,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<Vec<u32>> {
    if changes.len() == 1 {
        return changes[0].get_super_nodes(sc, tc);
    }

    let mut cc1 = changes[0].get_super_nodes(sc, tc);
    let cc2 = changes[1].get_super_nodes(sc, tc);

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

fn intersects(v1: &Vec<u32>, v2: &Vec<u32>) -> bool {
    for n in v1 {
        if v2.contains(&n) {
            return true;
        }
    }
    return false;
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
