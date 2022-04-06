use crate::classes::node_info::NodeInfo;
use crate::parser::meta::{MetaFile, Node, Supernode};
use std::collections::HashMap;

pub struct Meta {
    supernodes: HashMap<u32, Vec<u32>>,
    nodes: HashMap<u32, NodeInfo>,
}

impl Meta {
    pub fn new(supernodes: HashMap<u32, Vec<u32>>, nodes: HashMap<u32, NodeInfo>) -> Self {
        Self { supernodes, nodes }
    }

    pub fn to_file(&self) -> MetaFile {
        let mut s: Vec<Supernode> = Vec::new();
        let mut q: Vec<Node> = Vec::new();

        for (k, v) in self.supernodes {
            s.push(Supernode {
                i: k,
                g: v.to_vec(),
            });
        }

        for (k, v) in self.nodes {
            q.push(Node {
                i: k,
                p: v.parent,
                n: v.incoming.to_vec(),
                o: v.outgoing.to_vec(),
            });
        }
        return MetaFile { s, q };
    }

    pub fn from_file(file: MetaFile) -> Self {
        let mut supernodes: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut nodes: HashMap<u32, NodeInfo> = HashMap::new();

        for snode in file.s {
            supernodes.insert(snode.i, snode.g.to_vec());
        }

        for node in file.q {
            nodes.insert(node.i, NodeInfo::new(&node.p, &node.n, &node.o));
        }
        return Self::new(supernodes, nodes);
    }
}
