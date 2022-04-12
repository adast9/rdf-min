use crate::classes::dataset::Dataset;
use crate::classes::meta::Meta;
use crate::util;
use crate::Config;
use std::fs;

pub fn run(config: &Config, dataset: &Dataset, meta: &Meta) {
    util::io::write_triples(
        &config.dataset_path.parent().unwrap().join("summary.nt"),
        &dataset,
    )
    .unwrap();

    if config.use_fast {
        fs::create_dir(&config.meta_folder_path).unwrap();
    }

    util::io::write_dict(&config.meta_folder_path.join("dict"), &dataset).unwrap();

    util::io::write_meta(&config.meta_folder_path.join("meta.json"), &meta).unwrap();
}
