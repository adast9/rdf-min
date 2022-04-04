use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;

#[derive(Serialize, Deserialize)]
pub struct MetaFile {
    s: Vec<Supernode>,
    q: Vec<Node>,
}

impl MetaFile {
    pub fn new(supernodes: &HashMap<u32, Vec<u32>>, nodes: &HashMap<u32, NodeInfo>) -> Self {
        let mut s: Vec<Supernode> = Vec::new();
        let mut q: Vec<Node> = Vec::new();

        for (k, v) in supernodes {
            s.push(Supernode {
                i: *k as u32,
                g: v.to_vec(),
            });
        }

        for (k, v) in nodes {
            q.push(Node {
                i: *k as u32,
                p: v.parent,
                n: v.incoming.to_vec(),
                o: v.outgoing.to_vec(),
            });
        }
        MetaFile { s, q }
    }

    pub fn get_values(&self) -> Result<(HashMap<u32, Vec<u32>>, HashMap<u32, NodeInfo>), Error> {
        let mut supernodes: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut nodes: HashMap<u32, NodeInfo> = HashMap::new();

        for snode in &self.s {
            supernodes.insert(snode.i, snode.g.to_vec());
        }

        for node in &self.q {
            nodes.insert(node.i, NodeInfo::new(&node.p, &node.n, &node.o));
        }
        Ok((supernodes, nodes))
    }
}

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

#[derive(Serialize, Deserialize)]
struct Node {
    i: u32,
    p: Option<u32>,
    n: Vec<Vec<u32>>,
    o: Vec<Vec<u32>>,
}

#[derive(Serialize, Deserialize)]
struct Supernode {
    i: u32,
    g: Vec<u32>,
}
