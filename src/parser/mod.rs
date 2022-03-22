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

pub fn run(
    triple_path: &str,
    dict_path: &str,
    update_path: &str,
    meta_path: &str,
) -> Result<
    (
        HashMap<String, u32>,
        Vec<Triple>,
        Vec<Triple>,
        Vec<Triple>,
        Vec<Clique>,
        Vec<Clique>,
        HashMap<u32, [usize; 2]>,
        HashMap<u32, Vec<u32>>,
        HashMap<u32, NodeInfo>,
    ),
    std::io::Error,
> {
    let triple_lines = io::read_lines(triple_path)?;
    let dict = dict::parse_dict(&triple_lines, dict_path)?;
    let (triples, additions, deletions) = triple::get_triples(&triple_lines, update_path, &dict)?;
    let (source_clique, target_clique) = clique::create_cliques(&triples);
    let index_map = get_index_map(&source_clique, &target_clique);
    let (supernodes, nodes) = parse_meta(meta_path)?;

    Ok((
        dict,
        triples,
        additions,
        deletions,
        source_clique,
        target_clique,
        index_map,
        supernodes,
        nodes,
    ))
}
