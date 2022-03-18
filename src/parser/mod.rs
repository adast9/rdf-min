pub mod clique;
mod dict;
mod triple;

use self::{clique::Clique, triple::Triple};
use std::collections::HashMap;

pub fn run(
    triple_path: &str,
    dict_path: &str,
) -> Result<(HashMap<String, u32>, Vec<Triple>, Vec<Clique>, Vec<Clique>), std::io::Error> {
    let dict = dict::parse_dict(triple_path, dict_path)?;
    let triples = triple::push_triples_into_vector(triple_path, &dict)?;
    let (source_clique, target_clique) = clique::create_cliques(&triples);

    Ok((dict, triples, source_clique, target_clique))
}
