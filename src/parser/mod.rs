use crate::classes::clique::CliqueCollection;
use crate::classes::dataset::Dataset;
use crate::classes::meta::Meta;
use crate::Config;
pub mod clique;
pub mod dataset;
pub mod meta;

pub fn run(
    config: &Config,
) -> Result<(Dataset, Meta, CliqueCollection, CliqueCollection), std::io::Error> {
    let dataset = dataset::parse_dataset(&config)?;
    let meta = meta::parse_meta(&config)?;
    let (sc, tc) = clique::create_cliques(&dataset.triples.data_triples);

    Ok((dataset, meta, sc, tc))
}
