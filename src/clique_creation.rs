use crate::triple_parser::Triple;
use crate::clique::{Clique};

fn create_all_cliques(vec_of_triples: &Vec<Triple>) -> (Vec<Clique>, Vec<Clique>) {

    return (create_source_clique(vec_of_triples), create_target_clique(vec_of_triples));
}

fn create_source_clique(vec_of_triples: &Vec<Triple>) -> Vec<Clique> {
    let mut source_clique: Vec<Clique> = Vec::new();

    for triple in vec_of_triples.iter() {
        //Case: when no pred exists, then create a new clique
        match is_predicate_present(&source_clique, &triple) {
            false => { 
                source_clique.push(Clique::new(&vec!(triple.pred), &vec!(triple.sub)));
                continue;
            },
            true => (), 
        }

        //Case: when the predicate already exists, but not the subject, then put the subject into the clique of pred
        match index_of_pred(&source_clique, &triple) {
            Some(index) => {
                source_clique[index].nodes.push(triple.sub);
                continue;
            },
            None => (),
        }
        
        //Case: when the subject already exists, but not the predicate, then put the predicate into the clique of sub
        match index_of_node(&source_clique, &triple) {
            Some(index) => {
                source_clique[index].preds.push(triple.pred);
                continue;
            },
            None => (),
        }
    }
    return source_clique;
}

fn create_target_clique(vec_of_triples: &Vec<Triple>) -> Vec<Clique> {
    let mut target_clique: Vec<Clique> = Vec::new();

    for triple in vec_of_triples.iter() {
        //Case: when no pred exists, then create a new clique
        match is_predicate_present(&target_clique, &triple) {
            false => { 
                target_clique.push(Clique::new(&vec!(triple.pred), &vec!(triple.obj)));
                continue;
            },
            true => (), 
        }

        //Case: when the predicate already exists, but not the object, then put the object into the clique of pred
        match index_of_pred(&target_clique, &triple) {
            Some(index) => {
                target_clique[index].nodes.push(triple.obj);
                continue;
            },
            None => (),
        }
        
        //Case: when the object already exists, but not the predicate, then put the predicate into the clique of obj
        match index_of_node(&target_clique, &triple) {
            Some(index) => {
                target_clique[index].preds.push(triple.pred);
                continue;
            },
            None => (),
        }
    }
    return target_clique;
}

fn is_predicate_present(clique: &Vec<Clique>, triple: &Triple) -> bool {
    for c in clique {
        if !c.preds.contains(&triple.pred) {
            return false;
        }
    }
    return true;
}

fn index_of_pred(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.preds.contains(&triple.pred) {
            Some(index);
        }
    }
    return None;
}
fn index_of_node(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.sub) {
            Some(index);
        }
    }
    return None;
}