use crate::classes::dataset::Dataset;
use crate::classes::meta::Meta;
use crate::util::io;
use crate::Config;
use std::fs::create_dir;
use std::fs::remove_file;
use std::io::Error;
use std::path::PathBuf;

pub fn run(config: &Config, dataset: &Dataset, meta: &Meta) {
    write_triples(
        &config.dataset_path.parent().unwrap().join("summary.nt"),
        &dataset,
    )
    .unwrap();

    if config.use_fast {
        create_dir(&config.meta_folder_path).unwrap();
    }

    write_dict(&config.meta_folder_path.join("dict"), &dataset).unwrap();
    write_meta(&config.meta_folder_path.join("meta.json"), &meta).unwrap();
}

fn write_triples(path: &PathBuf, dataset: &Dataset) -> Result<(), Error> {
    let mut triple_strings: Vec<String> = Vec::new();

    for t in &dataset.triples.data_triples {
        triple_strings.push(t.to_string(dataset));
    }
    Ok(io::write_lines(path, &triple_strings)?)
}

fn write_dict(path: &PathBuf, dataset: &Dataset) -> Result<(), Error> {
    if path.exists() {
        remove_file(path)?;
    }
    Ok(io::write_lines(path, &dataset.dict_strings())?)
}

fn write_meta(path: &PathBuf, meta: &Meta) -> Result<(), Error> {
    let data = meta.serialize();
    let file_str = serde_json::to_string(&data)?;
    Ok(io::write_lines(path, &vec![file_str])?)
}
