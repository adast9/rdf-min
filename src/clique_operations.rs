use crate::triple_parser::Triple;
use crate::clique::Clique;

fn insert_new_triple(new_triple: Triple){

}

fn create_clique(vec_of_triples: &Vec<Triple>) -> (Vec<Clique>, Vec<Clique>) {
    let mut source_clique: Vec<Clique> = Vec::new();

    for triple in vec_of_triples {
        for clique in &source_clique {

        }

    }
    let mut target_clique: Vec<Clique> = Vec::new();
    return (source_clique, target_clique);
}