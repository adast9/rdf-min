mod parser;
mod tests;
mod util;

const TRIPLE_PATH: &str = "C:/dev/teriyaki/datasets/example.nt";
const DICT_PATH: &str = "C:/dev/teriyaki/meta/";

fn main() {
    let (dict, triples, source_clique, target_clique) =
        parser::run(TRIPLE_PATH, DICT_PATH).unwrap();
}
