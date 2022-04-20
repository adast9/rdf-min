use crate::classes::{clique::CliqueCollection, triple::Triple};

pub fn create_cliques(triples: &Vec<Triple>) -> (CliqueCollection, CliqueCollection) {
    let mut sc = CliqueCollection::new();
    let mut tc = CliqueCollection::new();

    for t in triples {
        sc.new_triple(&t.sub, &t.pred);
        tc.new_triple(&t.obj, &t.pred);
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

    for t in triples {
        if !ids.contains(&t.sub) {
            ids.push(t.sub);
        }
        if !ids.contains(&t.obj) && !t.is_type {
            ids.push(t.obj);
        }
    }
    return ids;
}
