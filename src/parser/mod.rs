pub mod clique;
mod dict;
mod triple;
mod update;

use self::{clique::Clique, triple::Triple};
use std::collections::HashMap;

pub fn run(
    triple_path: &str,
    dict_path: &str,
    update_path: &str,
) -> Result<(HashMap<String, u32>, Vec<Triple>, Vec<Clique>, Vec<Clique>), std::io::Error> {
    let dict = dict::parse_dict(triple_path, dict_path)?;

    // todo: make triples one function (triples, addition, deletions)
    let triples = triple::get_triples(triple_path, &dict)?;
    let (additions, deletions) = update::get_update_triples(update_path, &dict)?;
    let (source_clique, target_clique) = clique::create_cliques(&triples);

    Ok((dict, triples, source_clique, target_clique))
}
