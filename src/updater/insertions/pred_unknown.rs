use crate::{
    parser::{clique::Clique, triple::Triple, MetaData},
    updater::funcs::{add_unknown_pred_to_clique, update_index},
};

pub fn insert(stuff: &mut MetaData, triple: &Triple, sc: &mut Vec<Clique>, tc: &mut Vec<Clique>) {
    let sc_new = add_unknown_pred_to_clique(stuff, sc, &triple.pred, &triple.sub, 0);
    let tc_new = add_unknown_pred_to_clique(stuff, tc, &triple.pred, &triple.obj, 1);

    stuff.index_map.insert(triple.pred, [sc_new, tc_new]);
    if sc[sc_new].preds.len() == 1 {
        update_index(&mut stuff.index_map, &triple.sub, sc_new, 0);
    }
    if tc[tc_new].preds.len() == 1 {
        update_index(&mut stuff.index_map, &triple.obj, tc_new, 1);
    }
}
