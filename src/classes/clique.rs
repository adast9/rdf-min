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
