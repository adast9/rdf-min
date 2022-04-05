use super::{clique::Clique, triple::Triple};
use std::collections::{HashMap, VecDeque};

pub struct CliqueCollection {
    cliques: Vec<Clique>,
    queue: VecDeque<usize>,
    index_map: HashMap<u32, usize>,
}

impl CliqueCollection {
    pub fn new() -> Self {
        Self {
            cliques: vec![Clique::new(&vec![], &vec![])],
            queue: VecDeque::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn new_triple(&mut self, node: &u32, pred: &u32) {
        let node_exists = self.contains_node(&node);
        let pred_exists = self.contains_pred(&pred);

        if !node_exists && !pred_exists {
            self.new_clique(&vec![*node], &vec![*pred]);
        } else if !node_exists && pred_exists {
            self.add_node_to_clique(node, pred);
        } else if node_exists && !pred_exists {
            self.add_pred_to_clique(node, pred);
        } else {
            if self.index_map.get(node).unwrap() != self.index_map.get(pred).unwrap() {
                self.merge_cliques(node, pred);
            }
        }
    }

    /// Merges `b` into `a`, leaving `b` empty.
    /// Adds `b` to the queue.
    fn merge_cliques(&mut self, a: &u32, b: &u32) {
        let a_index = *self.index_map.get(a).unwrap();
        let b_index = *self.index_map.get(b).unwrap();

        let a_clique = &mut self.cliques[a_index];
        let b_clique = &mut self.cliques[b_index];

        self.set_index(&b_clique.nodes, &b_clique.preds, a_index);

        a_clique.nodes.append(&mut b_clique.nodes);
        a_clique.preds.append(&mut b_clique.preds);

        self.queue.push_back(b_index);
    }

    fn add_pred_to_clique(&mut self, node: &u32, pred: &u32) {
        let index = *self.index_map.get(node).unwrap();
        if index == 0 {
            panic!("Attempting to new pred to empty clique. wtf?")
        }

        self.cliques[index].preds.push(*pred);
        self.index_map.insert(*pred, index);
    }

    fn add_node_to_clique(&mut self, node: &u32, pred: &u32) {
        let index = *self.index_map.get(pred).unwrap();
        self.cliques[index].nodes.push(*node);
        self.index_map.insert(*node, index);
    }

    pub fn add_node_to_empty_clique(&mut self, node: &u32) {
        self.cliques[0].nodes.push(*node);
        self.index_map.insert(*node, 0);
    }

    fn clique_by_pred_mut(&self, pred: &u32) -> &mut Clique {
        if let Some(index) = self.index_map.get(pred) {
            return &mut &self.cliques[*index];
        }
        panic!("No clique found for predicate {}", pred);
    }

    fn new_clique(&mut self, nodes: &Vec<u32>, preds: &Vec<u32>) {
        if let Some(index) = self.queue.pop_front() {
            self.cliques[index] = Clique::new(&preds, &nodes);
            self.set_index(nodes, preds, index);
        } else {
            self.cliques.push(Clique::new(&preds, &nodes));
            self.set_index(nodes, preds, self.cliques.len() - 1);
        }
    }

    fn set_index(&mut self, nodes: &Vec<u32>, preds: &Vec<u32>, index: usize) {
        for n in nodes {
            self.index_map.insert(*n, index);
        }
        for p in preds {
            self.index_map.insert(*p, index);
        }
    }

    fn contains_pred(&self, pred: &u32) -> bool {
        return self.index_map.contains_key(pred);
    }

    /// sukma
    pub fn contains_node(&self, node: &u32) -> bool {
        return self.index_map.contains_key(node);
    }
}
