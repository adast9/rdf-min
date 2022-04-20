use crate::classes::{
    clique::CliqueChange, clique::CliqueCollection, dataset::Dataset, meta::Meta,
};
mod insertion;

pub fn run(
    dataset: &mut Dataset,
    meta: &mut Meta,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) {
    for i in 0..dataset.insertions.data_triples.len() {
        let changes = insertion::get_changes(
            &dataset.insertions.data_triples[i].clone(),
            dataset,
            meta,
            sc,
            tc,
        );

        if changes.is_empty() {
            continue;
        }

        let snodes = get_super_nodes(changes, sc, tc);
        apply_changes(dataset, meta, &snodes, sc, tc);
    }
}

pub fn get_super_nodes(
    changes: Vec<CliqueChange>,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) -> Vec<Vec<u32>> {
    if changes.len() == 1 {
        return changes[0].clone().get_super_nodes(sc, tc);
    }

    let mut snodes = changes[0].clone().get_super_nodes(sc, tc);
    snodes.extend(changes[1].clone().get_super_nodes(sc, tc));
    if !snodes.len() > 1 {
        return snodes;
    }

    let mut i = 0;
    let mut j = snodes.len() - 1;

    loop {
        if i == j {
            i += 1;
            if i == snodes.len() {
                break;
            }
            j = snodes.len();
        }

        if intersects(&snodes[i], &snodes[j]) {
            snodes[i] = union(&snodes[i], &snodes[j]);
            snodes.remove(j);
            j = snodes.len();
        } else {
            j -= 1;
        }
    }
    return snodes;
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

fn apply_changes(
    dataset: &mut Dataset,
    meta: &mut Meta,
    snodes: &Vec<Vec<u32>>,
    sc: &mut CliqueCollection,
    tc: &mut CliqueCollection,
) {
    for snode in snodes {
        let new_node = dataset.new_snode(snode, meta);
        meta.new_snode(snode, &new_node);
        sc.new_snode(snode, &new_node);
        tc.new_snode(snode, &new_node);
    }
}
