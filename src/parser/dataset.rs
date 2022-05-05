use crate::models::dataset::Dataset;
use crate::models::meta::Meta;
use crate::util::io;
use crate::Config;
use std::io::Error;
use std::path::PathBuf;

pub fn parse_dataset(config: &Config, meta: &mut Meta) -> Result<Dataset, Error> {
    let t_l = io::read_lines(&config.dataset_path)?;
    let (i_l, d_l) = get_update_lines(&config.update_path)?;
    if config.use_fast {
        Ok(Dataset::new(t_l, i_l, d_l, meta))
    } else {
        let dict_l = io::read_lines(&config.meta_folder_path.join("dict"))?;
        Ok(Dataset::new_with_dict(t_l, i_l, d_l, dict_l, meta))
    }
}

fn get_update_lines(update_path: &PathBuf) -> Result<(Vec<String>, Vec<String>), Error> {
    let mut i_l: Vec<String> = Vec::new();
    let mut d_l: Vec<String> = Vec::new();

    for l in io::read_lines(&update_path)? {
        // if l starts with '-', then it is a deletion
        let ch = l.chars().next().unwrap();

        if ch == '-' {
            let mut l = l;
            l.remove(0);
            d_l.push(l);
        } else {
            i_l.push(l);
        }
    }
    Ok((i_l, d_l))
}
