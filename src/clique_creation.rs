use crate::dict;
use crate::triple_parser::Triple;
use crate::clique::{Clique};

fn create_all_cliques(vec_of_triples: &Vec<Triple>) -> (Vec<Clique>, Vec<Clique>) {

    return (create_source_clique(vec_of_triples), create_target_clique(vec_of_triples));
}

fn get_all_distinct_nodes(vec_of_triples: &Vec<Triple>) -> Vec<u32> {
    let mut list_of_distinct_node: Vec<u32> = Vec::new();

    for triple in vec_of_triples {
        if !list_of_distinct_node.contains(&triple.sub) {
            list_of_distinct_node.push(triple.sub)
        }
        if !list_of_distinct_node.contains(&triple.obj) {
            list_of_distinct_node.push(triple.obj)
        }
    }

    return list_of_distinct_node;
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
        match index_of_sub(&source_clique, &triple) {
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
        match index_of_obj(&target_clique, &triple) {
            Some(index) => {
                target_clique[index].preds.push(triple.pred);
                continue;
            },
            None => (),
        }
    }
    return target_clique;
}

/* 
fn insert_triple(source_clique: &mut Vec<Clique>, target_clique: &mut Vec<Clique>, triple: &Triple) -> (Vec<Clique>, Vec<Clique>, Vec<u32>) {

    let x1 = index_of_sub(&source_clique, triple);
    let y1 = index_of_pred(&source_clique, triple);
    let x2 = index_of_obj(&target_clique, triple);
    let y2 = index_of_pred(&target_clique, triple);
    let new_super_node: Vec<u32> = Vec::new();
    
    if x1.is_some() && y1.is_some() && x2.is_some() {
        source_clique[x1.unwrap()].merge(&source_clique[y1.unwrap()]);
        target_clique[x2.unwrap()].merge(&target_clique[y2.unwrap()]);

        new_super_node = source_clique[x1.unwrap()].node_intersection(&target_clique[x2.unwrap()]);
    } else if x1.is_none() && y1.is_some() && x2.is_some() {
        source_clique[y1.unwrap()].nodes.push(x1.unwrap().try_into().unwrap());
        // do something.
    } else if x1.is_some() && y1.is_some() && x2.is_none() {
        // do something.
    }

    return (source_clique, target_clique, new_super_node);
}*/

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
            return Some(index);
        }
    }
    return None;
}

fn index_of_sub(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.sub) {
            return Some(index);
        }
    }
    return None;
}

fn index_of_obj(clique: &Vec<Clique>, triple: &Triple) -> Option<usize> {
    for (index, c) in clique.iter().enumerate() {
        if c.nodes.contains(&triple.obj) {
           return Some(index);
        }
    }
    return None;
}