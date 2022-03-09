fn main() {
    let str = "sub pred obj .";
    let v: Vec<&str> = str.split(' ').collect();

    for i in 0..3 {
        println!("{}", v[i]);
    }
}
