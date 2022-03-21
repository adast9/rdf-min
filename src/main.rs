mod parser;
mod tests;
mod util;

const TRIPLE_PATH: &str = "C:/dev/teriyaki/datasets/example.nt";
const DICT_PATH: &str = "C:/dev/teriyaki/meta/";
const UPDATE_PATH: &str = "C:/dev/teriyaki/meta/"; // todo: get these paths nice

fn main() {
    let (dict, triples, source_clique, target_clique) =
        parser::run(TRIPLE_PATH, DICT_PATH, UPDATE_PATH).unwrap();
}
