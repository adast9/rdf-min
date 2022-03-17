mod dict;
mod triple_parser;
mod meta_parser;
mod clique;
mod clique_creation;
mod io_helper;

fn main() {
    let dict = dict::Dictionary::new("./example.nt", "./dict").unwrap();

    println!("{:?}", dict.get_dict());
    println!("{}", dict.get("<http://p6.gov/course>").unwrap());
}
