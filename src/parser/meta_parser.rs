use crate::parser::meta::{MetaFile, NodeInfo};
use crate::Config;
use std::collections::HashMap;
use std::fs;
use std::io;

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
