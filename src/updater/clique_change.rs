pub struct Clique_Change {
    clique_index: usize,
    new_nodes: Vec<u32>,
    is_source: bool,
}

impl Clique_Change {
    pub fn new(clique_index: usize, new_nodes: Vec<u32>, is_source: bool) -> Self {
        Self {
            clique_index,
            new_nodes,
            is_source,
        }
    }
}
