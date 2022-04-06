use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub struct Clique {
    pub preds: Vec<u32>,
    pub nodes: Vec<u32>,
}

impl Clique {
    /// Creates a new `Clique` with the given `preds` and `nodes`.
    pub fn new(preds: &Vec<u32>, nodes: &Vec<u32>) -> Self {
        Self {
            preds: preds.clone(),
            nodes: nodes.clone(),
        }
    }

    /// Creates an empty `Clique`.
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

    /// Returns a `Vec` of all nodes contained both in `self` and `c`.
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

pub struct CliqueCollection {
    cliques: Vec<Clique>,
    queue: VecDeque<usize>,
    index_map: HashMap<u32, usize>,
}

impl CliqueCollection {
    /// Creates an empty `CliqueCollection`.
    ///
    /// Initially there is only the empty clique.
    pub fn new() -> Self {
        Self {
            cliques: vec![Clique::new(&vec![], &vec![])],
            queue: VecDeque::new(),
            index_map: HashMap::new(),
        }
    }

    /// Adds the `node` and `pred` of a new triple to the `CliqueCollection`.
    ///
    /// `node` and `pred` do not have to been previosly known.
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

    /// Merges the cliques containing the ids `a` and `b`, which can either be nodes or preds.
    ///
    /// `b`'s clique is merged into `a`'s clique, leaving `b`'s clique empty.
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

    /// Adds `pred` to the clique containing `node`.
    ///
    /// # Panics
    ///
    /// Panics if `node` is in the empty clique.
    fn add_pred_to_clique(&mut self, node: &u32, pred: &u32) {
        let index = *self.index_map.get(node).unwrap();
        if index == 0 {
            panic!("Attempting to add new pred to empty clique. wtf?")
        }

        self.cliques[index].preds.push(*pred);
        self.index_map.insert(*pred, index);
    }

    /// Adds `node` to the clique containing `pred`.
    fn add_node_to_clique(&mut self, node: &u32, pred: &u32) {
        let index = *self.index_map.get(pred).unwrap();
        self.cliques[index].nodes.push(*node);
        self.index_map.insert(*node, index);
    }

    /// Adds the node `node` to the empty clique.
    pub fn add_node_to_empty_clique(&mut self, node: &u32) {
        self.cliques[0].nodes.push(*node);
        self.index_map.insert(*node, 0);
    }

    /// Returns a mutable reference to the clique containing `pred`.
    ///
    /// # Panics
    ///
    /// Panics if the `CliqueCollection` does not contain a clique with `pred`.
    fn clique_by_pred_mut(&self, pred: &u32) -> &mut Clique {
        if let Some(index) = self.index_map.get(pred) {
            return &mut &self.cliques[*index];
        }
        panic!("No clique found for predicate {}", pred);
    }

    /// Adds a new clique to the `CliqueCollection` containing `nodes` and `preds`.
    fn new_clique(&mut self, nodes: &Vec<u32>, preds: &Vec<u32>) {
        if let Some(index) = self.queue.pop_front() {
            self.cliques[index] = Clique::new(&preds, &nodes);
            self.set_index(nodes, preds, index);
        } else {
            self.cliques.push(Clique::new(&preds, &nodes));
            self.set_index(nodes, preds, self.cliques.len() - 1);
        }
    }

    /// Sets the indices of `nodes` and `preds` to `index`.
    fn set_index(&mut self, nodes: &Vec<u32>, preds: &Vec<u32>, index: usize) {
        for n in nodes {
            self.index_map.insert(*n, index);
        }
        for p in preds {
            self.index_map.insert(*p, index);
        }
    }

    /// Returns true if the `CliqueCollection` contains a clique with `pred`.
    fn contains_pred(&self, pred: &u32) -> bool {
        return self.index_map.contains_key(pred);
    }

    /// Returns true if the `CliqueCollection` contains a clique with `node`.
    pub fn contains_node(&self, node: &u32) -> bool {
        return self.index_map.contains_key(node);
    }
}
