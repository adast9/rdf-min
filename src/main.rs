mod dict;

fn main() {
    let dict = dict::Dictionary::new("./example.nt", "./dict");

    println!("{:?}", dict.get_dict());
    println!("{}", dict.get("<http://p6.gov/course>").unwrap());
}
