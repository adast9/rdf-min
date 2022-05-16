use crate::{
    models::{
        clique::CliqueChange, clique::CliqueCollection, dataset::Dataset, meta::Meta,
        triple::Triple,
    },
    util::set_ops::get_disjoint_sets,
};
mod deletion;
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
        let changes = deletion::delete_triple(
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

    add_types_to_dataset(dataset, meta);
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

fn add_types_to_dataset(dataset: &mut Dataset, meta: &mut Meta) {
    let type_pred =
        dataset.get_from_dict("<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>".to_string());
    for [s, o] in meta.get_types() {
        if meta.contains(s) {
            dataset.triples.add_data_triple(&Triple::new(
                meta.get_parent(s).unwrap_or(*s),
                type_pred,
                *o,
                true,
            ));
        } else {
            dataset
                .triples
                .add_data_triple(&Triple::new(*s, type_pred, *o, true));
        }
    }
}
