pub mod clique;
mod dict;
pub(crate) mod triple;
use crate::util::io;

use self::{clique::Clique, triple::Triple};
use std::collections::HashMap;

pub fn run(
    triple_path: &str,
    dict_path: &str,
    update_path: &str,
) -> Result<
    (
        HashMap<String, u32>,
        Vec<Triple>,
        Vec<Triple>,
        Vec<Triple>,
        Vec<Clique>,
        Vec<Clique>,
    ),
    std::io::Error,
> {
    let triple_lines = io::read_lines(triple_path)?;
    let dict = dict::parse_dict(&triple_lines, dict_path)?;
    let (triples, additions, deletions) = triple::get_triples(&triple_lines, update_path, &dict)?;
    let (source_clique, target_clique) = clique::create_cliques(&triples);

    Ok((
        dict,
        triples,
        additions,
        deletions,
        source_clique,
        target_clique,
    ))
}
