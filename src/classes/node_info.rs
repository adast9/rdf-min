pub struct NodeInfo {
    // todo getters and setters?
    pub parent: Option<u32>,
    // todo: incoming and outgoing should be Vec<[u32;2]>
    pub incoming: Vec<Vec<u32>>,
    pub outgoing: Vec<Vec<u32>>,
}

impl NodeInfo {
    pub fn new(parent: &Option<u32>, incoming: &Vec<Vec<u32>>, outgoing: &Vec<Vec<u32>>) -> Self {
        NodeInfo {
            parent: parent.clone(),
            incoming: incoming.clone(),
            outgoing: outgoing.clone(),
        }
    }
}
