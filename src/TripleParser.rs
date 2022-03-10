use std::fs::File;
use std::io::{BufRead, BufReader};

struct Triple {
	subjectID: u32,
	predicateID: u32,
    objectID: u32,
    isType: bool
}

fn file_reader() -> BufReader<R> {
    let filename = "/Users/alankhorsid/Documents/Datalogi/6. semester/P6/teriyaki/datasets/example.nt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader;
}

fn push_triples_into_vector(reader: BufReader<R>) -> Result<Vec<Triple>, io::Error> {
    let mut vector_of_triples: Vec<Triple> = Vec::new();
    let is_type_pred = false;

    for line in reader.lines() {
        let line = line.unwrap();

        let line_splits: Vec<&str> = line.split(" ").collect();

        if line_splits[3].contains("type") {
            is_type_pred = true;
        }
        //GetID does not exist. Its pseudo 
        vector_of_triples.push(triple{subjectID: GetID(line_splits[0]), predicateID: GetID(line_splits[1]), objectID: GetID(line_splits[2]), isType: is_type_pred});
        //println!("{}", line_splits[i])
    }
    OK(vector_of_triples);
}