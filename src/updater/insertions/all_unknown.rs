use crate::{
    parser::{clique::Clique, meta_parser::NodeInfo, triple::Triple, MetaData},
    updater::funcs::{add_id_to_index_map, add_unknown_node_and_pred_to_clique},
};

pub fn insert(stuff: &mut MetaData, triple: &Triple, sc: &mut Vec<Clique>, tc: &mut Vec<Clique>) {
    add_unknown_node_and_pred_to_clique(triple.sub, triple.pred, sc, tc);
    add_unknown_node_and_pred_to_clique(triple.obj, triple.pred, tc, sc);

    add_id_to_index_map(&mut stuff.index_map, &triple.sub, sc, tc);
    add_id_to_index_map(&mut stuff.index_map, &triple.pred, sc, tc);
    add_id_to_index_map(&mut stuff.index_map, &triple.obj, sc, tc);

    stuff.nodes.insert(
        triple.sub,
        NodeInfo::new(&None, &vec![], &vec![vec![triple.pred, triple.obj]]),
    );
    stuff.nodes.insert(
        triple.obj,
        NodeInfo::new(&None, &vec![vec![triple.pred, triple.sub]], &vec![]),
    );
}
