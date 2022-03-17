mod dict;
mod triple_parser;
mod meta_parser;
mod clique;
mod index_map;
mod clique_tests;
// mod clique_operations;
mod clique_creation;
mod io_helper;

const TRIPLE_PATH: &str = "C:/dev/teriyaki/datasets/example.nt";
const DICT_PATH: &str = "C:/dev/teriyaki/meta/dict";
const META_PATH: &str = "C:/dev/teriyaki/meta/meta2.json";

fn main() {
    let dict = dict::Dictionary::new(TRIPLE_PATH, DICT_PATH).unwrap();
    let (supernodes, nodes) = meta_parser::parse_meta(META_PATH).unwrap();
    let triples = triple_parser::push_triples_into_vector(TRIPLE_PATH, dict.get_dict()).unwrap();
    let (source_clique, target_clique) = clique_creation::create_cliques(&triples);

    println!("{:?}", dict.get_dict());

    println!("SOURCE");
    for clique in source_clique {
        print!("{:?} - ", clique.preds);
        println!("{:?}", clique.nodes);
    }

    println!("TARGET");
    for clique in target_clique {
        print!("{:?} - ", clique.preds);
        println!("{:?}", clique.nodes);
    }
}
