use crate::parser::triple::Triple;

#[derive(Clone)]
pub struct Clique {
    pub preds: Vec<u32>,
    pub nodes: Vec<u32>,
}

impl Clique {
    pub fn new(preds: &Vec<u32>, nodes: &Vec<u32>) -> Self {
        Self {
            preds: preds.clone(),
            nodes: nodes.clone(),
        }
    }

    pub fn empty() -> Self {
        Self {
            preds: vec![],
            nodes: vec![],
        }
    }

    /// Adds `node` to the nodes of the `Clique`.
    pub fn add_node(&mut self, node: &u32) {
        self.nodes.push(*node);
    }

    /// Removes `node` from the nodes of the `Clique`.
    pub fn remove_node(&mut self, node: &u32) {
        self.nodes.retain(|n| *n != *node);
    }

    pub fn merge(&mut self, c: &Clique) {
        self.preds.append(&mut c.preds.clone());
        self.nodes.append(&mut c.nodes.clone());
    }

    pub fn node_intersection(&self, c: &Clique) -> Vec<u32> {
        let mut intersection: Vec<u32> = Vec::new();

        for node in &self.nodes {
            if c.nodes.contains(&node) {
                intersection.push(node.clone());
            }
        }

        return intersection;
    }
}

pub fn create_cliques(triples: &Vec<Triple>) -> (Vec<Clique>, Vec<Clique>) {
    let mut source_cliques: Vec<Clique> = Vec::new();
    let mut target_cliques: Vec<Clique> = Vec::new();

    // Add empty sets
    source_cliques.push(Clique::new(&vec![], &vec![]));
    target_cliques.push(Clique::new(&vec![], &vec![]));

    for triple in triples {
        add_new_triple(triple, &mut source_cliques, true);
        add_new_triple(triple, &mut target_cliques, false);
    }

    let ids = unique_ids(triples);

    for id in ids {
        let mut id_found = false;
        for c in &source_cliques {
            if c.nodes.contains(&id) {
                id_found = true;
                break;
            }
        }
        if !id_found {
            let len = source_cliques.len();
            source_cliques[len - 1].nodes.push(id);
        }

        id_found = false;
        for c in &target_cliques {
            if c.nodes.contains(&id) {
                id_found = true;
                break;
            }
        }
        if !id_found {
            let len = target_cliques.len();
            target_cliques[len - 1].nodes.push(id);
        }
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

pub fn index_of_pred(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.preds.contains(&triple.pred) {
            return Some(index);
        }
    }
    return None;
}

pub fn index_of_sub(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.sub) {
            return Some(index);
        }
    }
    return None;
}

pub fn index_of_obj(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.obj) {
            return Some(index);
        }
    }
    return None;
}

fn unique_ids(triples: &Vec<Triple>) -> Vec<u32> {
    let mut ids: Vec<u32> = Vec::new();

    for triple in triples {
        if !ids.contains(&triple.sub) {
            ids.push(triple.sub);
        }
        if !ids.contains(&triple.obj) && !triple.is_type {
            ids.push(triple.obj);
        }
    }

    ids
}
