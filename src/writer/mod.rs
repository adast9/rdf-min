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

    for triple in &dataset.triples.data_triples {
        let sub_string = dataset.key_by_value(&triple.sub).unwrap();
        let pred_string = dataset.key_by_value(&triple.pred).unwrap();
        let obj_string = dataset.key_by_value(&triple.obj).unwrap();

        let triple_string = format!("{} {} {} .", sub_string, pred_string, obj_string);

        if !triple_strings.contains(&triple_string) {
            triple_strings.push(triple_string);
        }
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
