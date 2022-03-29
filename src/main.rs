mod parser;
mod tests;
mod updater;
mod util;

const TRIPLE_PATH: &str = "C:/dev/teriyaki/datasets/example.nt";
const DICT_PATH: &str = "C:/dev/teriyaki/meta/dict";
const UPDATE_PATH: &str = "C:/dev/teriyaki/meta/update.nt"; // todo: get these paths nice
const META_PATH: &str = "C:/dev/teriyaki/meta/meta.json";

fn main() {
    let (mut stuff, additions, deletions, mut sc, mut tc) =
        parser::run(TRIPLE_PATH, DICT_PATH, UPDATE_PATH, META_PATH).unwrap();

    println!("SOURCE CLIQUES");
    util::print::cliques_string(&sc, &stuff.dict);
    println!("");
    println!("TARGET CLIQUES");
    util::print::cliques_string(&tc, &stuff.dict);
    println!("");
    println!("  |          |          |          |          |");
    println!("  v          v          v          v          v");
    println!("");

    updater::run(&mut stuff, additions, deletions, &mut sc, &mut tc);

    println!("SOURCE CLIQUES");
    util::print::cliques_string(&sc, &stuff.dict);
    println!("");
    println!("TARGET CLIQUES");
    util::print::cliques_string(&tc, &stuff.dict);
}
