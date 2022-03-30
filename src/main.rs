use std::{
    env,
    path::{Path, PathBuf},
    process,
};

mod parser;
mod tests;
mod updater;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let (mut stuff, additions, deletions, mut sc, mut tc) = parser::run(&config).unwrap();

    updater::run(&mut stuff, additions, deletions, &mut sc, &mut tc);

    util::io::write_triples(
        &config.dataset_path.parent().unwrap().join("summary.nt"),
        &stuff.triples,
        &stuff.dict,
    )
    .unwrap();

    println!("SOURCE CLIQUES");
    util::print::cliques_string(&sc, &stuff.dict);
    println!("");
    println!("TARGET CLIQUES");
    util::print::cliques_string(&tc, &stuff.dict);

    // println!("");
    // println!("TRIPLES");
    // util::print::triples_string(&stuff.triples, &stuff.dict);
}

pub struct Config {
    dataset_path: PathBuf,
    meta_folder_path: PathBuf,
    update_path: PathBuf,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let dataset_path = PathBuf::from(&args[1]);
        if !dataset_path.exists() {
            return Err("dataset path does not exist");
        }

        let meta_folder_path = PathBuf::from(&args[2]);
        if !meta_folder_path.exists() {
            return Err("meta folder path does not exist");
        }

        let update_path = PathBuf::from(&args[3]);
        if !update_path.exists() {
            return Err("update path does not exist");
        }

        Ok(Config {
            dataset_path,
            meta_folder_path,
            update_path,
        })
    }
}
