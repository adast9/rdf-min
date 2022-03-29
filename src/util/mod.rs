use std::collections::HashMap;

pub mod io;
pub mod print;

pub fn generate_new_id(dict: &HashMap<String, u32>) -> u32 {
    let mut biggest_key: u32 = 0;
    for x in dict.keys() {
        if biggest_key < *dict.get(x).unwrap() {
            biggest_key = *dict.get(x).unwrap();
        }
    }
    return biggest_key + 1;
}
