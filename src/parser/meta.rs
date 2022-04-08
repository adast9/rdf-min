use crate::classes::meta::Meta;
use crate::classes::meta::NodeInfo;
use crate::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

pub fn parse_meta(config: &Config) -> Result<Meta, io::Error> {
    if config.use_fast {
        let supernodes: HashMap<u32, Vec<u32>> = HashMap::new();
        let nodes: HashMap<u32, NodeInfo> = HashMap::new();
        Ok(Meta::new(supernodes, nodes))
    } else {
        let file_str = fs::read_to_string(&config.meta_folder_path.join("meta.json"))?;
        let file_data: MetaFile = serde_json::from_str(&file_str)?;
        Ok(Meta::deserialize(file_data))
    }
}

#[derive(Serialize, Deserialize)]
pub struct MetaFile {
    pub s: Vec<Supernode>,
    pub q: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub i: u32,
    pub p: Option<u32>,
    pub n: Vec<Vec<u32>>,
    pub o: Vec<Vec<u32>>,
}

#[derive(Serialize, Deserialize)]
pub struct Supernode {
    pub i: u32,
    pub g: Vec<u32>,
}
