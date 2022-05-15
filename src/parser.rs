use crate::models::clique::CliqueCollection;
use crate::models::dataset::Dataset;
use crate::models::meta::Meta;
use crate::Config;
pub mod clique;
pub mod dataset;
pub mod meta;

pub fn run(
    config: &Config,
) -> Result<(Dataset, Meta, CliqueCollection, CliqueCollection), std::io::Error> {
    let mut meta = meta::parse_meta(&config)?;
    let dataset = dataset::parse_dataset(&config, &mut meta)?;
    let (sc, tc) = clique::create_cliques(&dataset.triples.data_triples);

    Ok((dataset, meta, sc, tc))
}
