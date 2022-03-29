use crate::{
    parser::{clique::Clique, triple::Triple, Stuff},
    updater::funcs::add_unknown_node_and_pred_to_clique,
};

pub fn insert(stuff: &mut Stuff, triple: &Triple, sc: &mut Vec<Clique>, tc: &mut Vec<Clique>) {
    add_unknown_node_and_pred_to_clique(triple.sub, triple.pred, sc, tc);
    add_unknown_node_and_pred_to_clique(triple.obj, triple.pred, tc, sc);
}
