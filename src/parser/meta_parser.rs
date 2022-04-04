use crate::parser::meta::{MetaFile, NodeInfo};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn parse_meta(
    path: &PathBuf,
) -> Result<(HashMap<u32, Vec<u32>>, HashMap<u32, NodeInfo>), io::Error> {
    let file_str = fs::read_to_string(path)?;
    let file_data: MetaFile = serde_json::from_str(&file_str)?;
    Ok(file_data.get_values()?)
}
