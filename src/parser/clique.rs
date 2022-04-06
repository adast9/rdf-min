use crate::classes::{clique::CliqueCollection, triple::Triple};

pub fn create_cliques(triples: &Vec<Triple>) -> (CliqueCollection, CliqueCollection) {
    let sc = CliqueCollection::new();
    let tc = CliqueCollection::new();

    for triple in triples {
        sc.new_triple(&triple.sub, &triple.pred);
        tc.new_triple(&triple.obj, &triple.pred);
    }

    for n in unique_nodes(triples) {
        if !sc.contains_node(&n) {
            sc.add_node_to_empty_clique(&n);
        }
        if !tc.contains_node(&n) {
            tc.add_node_to_empty_clique(&n);
        }
    }
    return (sc, tc);
}

fn unique_nodes(triples: &Vec<Triple>) -> Vec<u32> {
    // todo: move into Triples struct
    let mut ids: Vec<u32> = Vec::new();

    for triple in triples {
        if !ids.contains(&triple.sub) {
            ids.push(triple.sub);
        }
        if !ids.contains(&triple.obj) && !triple.is_type {
            ids.push(triple.obj);
        }
    }
    return ids;
}
