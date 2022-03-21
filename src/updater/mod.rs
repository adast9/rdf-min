use crate::updater::index_map::{get_index_map};
mod index_map;
mod meta_parser;
use crate::parser::clique::Clique;
use crate::parser::triple::Triple;


fn add_new_triple( 
    new_triple: Triple,
    source_clique: &mut Vec<Clique>, 
    target_clique: &mut Vec<Clique> 
) {

    let index_map = get_index_map(source_clique, target_clique);

    let sub_val = index_map.get(&new_triple.sub).unwrap();
    let pred_val = index_map.get(&new_triple.pred).unwrap();

    source_clique[sub_val[0] as usize].nodes.append(&mut source_clique[pred_val[0] as usize].nodes);
    source_clique[sub_val[0] as usize].preds.append(&mut source_clique[pred_val[0] as usize].preds);

}