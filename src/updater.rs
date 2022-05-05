use crate::{
    models::{clique::CliqueChange, clique::CliqueCollection, dataset::Dataset, meta::Meta},
    util::set_ops::get_disjoint_sets,
};
mod deletion2;
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

    for i in 0..dataset.deletions.data_triples.len() {
        let changes = deletion2::delete_triple(
            &dataset.deletions.data_triples[i].clone(),
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

    for i in 0..dataset.insertions.type_triples.len() {
        if !meta.contains(&dataset.insertions.type_triples[i].sub) {
            meta.new_node(&dataset.insertions.type_triples[i], true)
        } else {
            meta.add_outgoing(&dataset.insertions.type_triples[i]);
        }
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

    return get_disjoint_sets(snodes);
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
