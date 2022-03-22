
pub fn vec_intersect(
    v1: Vec<u32>,
    v2: Vec<u32>
) -> Vec<u32> {

    let mut intersection: Vec<u32> = Vec::new();

    for id in v1 {
        if v2.contains(&id) {intersection.push(id);}
    }

    intersection
}

pub fn node_intersect(
    &self, 
    c: &Clique
) -> Vec<u32> {
    let mut intersection: Vec<u32> = Vec::new();

    for node in &self.nodes {
        if c.nodes.contains(&node) {
            intersection.push(node.clone());
        }
    }

    intersection
}