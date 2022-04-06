use crate::classes::dict::Dict;
use crate::util::io;
use crate::Config;
use std::io::Error;
use std::path::PathBuf;

pub fn parse_dict(triple_lines: &Vec<String>, config: &Config) -> Result<Dict, Error> {
    if config.use_fast {
        Ok(gen_dict(triple_lines)?)
    } else {
        let dict_path = config.meta_folder_path.join("dict");
        Ok(read_dict(&dict_path)?)
    }
}

fn read_dict(dict_path: &PathBuf) -> Result<Dict, Error> {
    let mut dict = Dict::new();
    for l in io::read_lines(dict_path)? {
        dict.add(&l);
    }
    Ok(dict)
}

fn gen_dict(triple_lines: &Vec<String>) -> Result<Dict, Error> {
    let mut dict = Dict::new();
    for l in triple_lines {
        let v: Vec<&str> = l.split(' ').collect();
        for i in 0..3 {
            let str = v[i].to_string();
            if !dict.contains(&str) {
                dict.add(&str);
            }
        }
    }
    Ok(dict)
}
