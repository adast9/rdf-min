use crate::clique::Clique;
use crate::dict;
use crate::triple_parser::Triple;

pub(crate) fn create_cliques(triples: &Vec<Triple>) -> (Vec<Clique>, Vec<Clique>) {
    let mut source_cliques: Vec<Clique> = Vec::new();
    let mut target_cliques: Vec<Clique> = Vec::new();

    for triple in triples {
        add_new_triple(triple, &mut source_cliques, true);
        add_new_triple(triple, &mut target_cliques, false);
    }

    return (source_cliques, target_cliques);
}

fn add_new_triple(triple: &Triple, cliques: &mut Vec<Clique>, is_source: bool) {
    if triple.is_type {
        return;
    };

    let pred_index = index_of_pred(&cliques, &triple);
    let node_index = if is_source {
        index_of_sub(&cliques, &triple)
    } else {
        index_of_obj(&cliques, &triple)
    };
    // If both pred and node are new - Add new clique
    if pred_index == None && node_index == None {
        if is_source {
            cliques.push(Clique::new(&vec![triple.pred], &vec![triple.sub]));
        } else {
            cliques.push(Clique::new(&vec![triple.pred], &vec![triple.obj]));
        }
    }
    // If only pred is new - Push pred to preds in already existing clique
    else if pred_index == None {
        cliques[node_index.unwrap()].preds.push(triple.pred);
    }
    // If only node is new - Push sub to nodes in alreayd existing clique
    else if node_index == None {
        cliques[pred_index.unwrap()]
            .nodes
            .push(if is_source { triple.sub } else { triple.obj });
    }
    // If none are new
    else {
        let pred_i = pred_index.unwrap();
        let node_i = node_index.unwrap();

        // If they are not in the same clique - Merge cliques
        if pred_i != node_i {
            let mut pred_clique = cliques[pred_i].clone();
            let sub_clique = cliques[node_i].clone();
            pred_clique.merge(&sub_clique);

            cliques[pred_i] = pred_clique;
            cliques.remove(node_i);
        }

        // If they are in the same clique - Do nothing
    }
}

fn index_of_pred(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.preds.contains(&triple.pred) {
            return Some(index);
        }
    }
    return None;
}

fn index_of_sub(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.sub) {
            return Some(index);
        }
    }
    return None;
}

fn index_of_obj(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.obj) {
            return Some(index);
        }
    }
    return None;
}
