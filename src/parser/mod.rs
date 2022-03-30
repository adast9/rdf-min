pub mod clique;
pub mod dict;
pub mod index_map;
pub mod meta_parser;
pub mod triple;
use crate::{util::io, Config};

use self::{
    clique::Clique,
    index_map::get_index_map,
    meta_parser::{parse_meta, NodeInfo},
    triple::Triple,
};
use std::{collections::HashMap, path::PathBuf};

pub struct MetaData {
    pub dict: HashMap<String, u32>,
    pub triples: Vec<Triple>,
    pub index_map: HashMap<u32, [usize; 2]>,
    pub supernodes: HashMap<u32, Vec<u32>>,
    pub nodes: HashMap<u32, NodeInfo>,
}

impl MetaData {
    fn new(
        dict: HashMap<String, u32>,
        triples: Vec<Triple>,
        index_map: HashMap<u32, [usize; 2]>,
        supernodes: HashMap<u32, Vec<u32>>,
        nodes: HashMap<u32, NodeInfo>,
    ) -> Self {
        Self {
            dict,
            triples,
            index_map,
            supernodes,
            nodes,
        }
    }
}

pub fn run(
    config: &Config,
) -> Result<(MetaData, Vec<Triple>, Vec<Triple>, Vec<Clique>, Vec<Clique>), std::io::Error> {
    let triple_lines = io::read_lines(&config.dataset_path)?;
    let mut dict = dict::parse_dict(&triple_lines, &config.meta_folder_path.join("dict"))?;
    let (triples, additions, deletions) =
        triple::get_triples(&triple_lines, &config.update_path, &mut dict)?;
    let (source_cliques, target_cliques) = clique::create_cliques(&triples);
    let index_map = get_index_map(&source_cliques, &target_cliques);
    let (supernodes, nodes) = parse_meta(&config.meta_folder_path.join("meta.json"))?;

    let stuff = MetaData::new(dict, triples, index_map, supernodes, nodes);

    Ok((stuff, additions, deletions, source_cliques, target_cliques))
}
