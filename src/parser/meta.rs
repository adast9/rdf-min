use crate::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Error;

pub fn parse_meta(
    config: &Config,
) -> Result<(HashMap<u32, Vec<u32>>, HashMap<u32, NodeInfo>), io::Error> {
    if config.use_fast {
        let supernodes: HashMap<u32, Vec<u32>> = HashMap::new();
        let nodes: HashMap<u32, NodeInfo> = HashMap::new();
        Ok((supernodes, nodes))
    } else {
        let file_str = fs::read_to_string(&config.meta_folder_path.join("meta.json"))?;
        let file_data: MetaFile = serde_json::from_str(&file_str)?;
        Ok(file_data.get_values()?)
    }
}

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
