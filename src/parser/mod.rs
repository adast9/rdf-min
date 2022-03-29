pub mod clique;
pub mod dict;
pub mod index_map;
pub mod meta_parser;
pub mod triple;
use crate::util::io;

use self::{
    clique::Clique,
    index_map::get_index_map,
    meta_parser::{parse_meta, NodeInfo},
    triple::Triple,
};
use std::collections::HashMap;

pub struct Stuff {
    pub dict: HashMap<String, u32>,
    pub triples: Vec<Triple>,
    pub index_map: HashMap<u32, [usize; 2]>,
    pub supernodes: HashMap<u32, Vec<u32>>,
    pub nodes: HashMap<u32, NodeInfo>,
}

impl Stuff {
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
    triple_path: &str,
    dict_path: &str,
    update_path: &str,
    meta_path: &str,
) -> Result<(Stuff, Vec<Triple>, Vec<Triple>, Vec<Clique>, Vec<Clique>), std::io::Error> {
    let triple_lines = io::read_lines(triple_path)?;
    let dict = dict::parse_dict(&triple_lines, dict_path)?;
    let (triples, additions, deletions) = triple::get_triples(&triple_lines, update_path, &dict)?;
    let (source_cliques, target_cliques) = clique::create_cliques(&triples);
    let index_map = get_index_map(&source_cliques, &target_cliques);
    let (supernodes, nodes) = parse_meta(meta_path)?;

    let stuff = Stuff::new(dict, triples, index_map, supernodes, nodes);

    Ok((stuff, additions, deletions, source_cliques, target_cliques))
}
