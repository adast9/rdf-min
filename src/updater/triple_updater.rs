use crate::classes::clique::CliqueCollection;
use crate::classes::dataset::Dataset;
use crate::classes::meta::Meta;

pub fn update_changes(
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
