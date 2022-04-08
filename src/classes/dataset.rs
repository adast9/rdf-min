use crate::classes::dict::Dict;
use crate::classes::triple::TripleCollection;

use super::meta::Meta;
use super::triple::Triple;

pub struct Dataset {
    dict: Dict,
    pub triples: TripleCollection,
    pub insertions: TripleCollection,
    pub deletions: TripleCollection,
}

impl Dataset {
    pub fn new(t_l: Vec<String>, i_l: Vec<String>, d_l: Vec<String>) -> Self {
        let mut dict = Dict::empty();
        let triples = TripleCollection::new(t_l, &mut dict);
        let insertions = TripleCollection::new(i_l, &mut dict);
        let deletions = TripleCollection::new(d_l, &mut dict);

        Self {
            dict,
            triples,
            insertions,
            deletions,
        }
    }

    pub fn new_with_dict(
        t_l: Vec<String>,
        i_l: Vec<String>,
        d_l: Vec<String>,
        dict_l: Vec<String>,
    ) -> Self {
        let mut dict = Dict::new(&dict_l);
        let triples = TripleCollection::new(t_l, &mut dict);
        let insertions = TripleCollection::new(i_l, &mut dict);
        let deletions = TripleCollection::new(d_l, &mut dict);

        Self {
            dict,
            triples,
            insertions,
            deletions,
        }
    }

    pub fn add_triple(&mut self, triple: Triple, meta: &Meta) {
        if let Some(p) = meta.get_parent(&triple.sub) {
            triple.sub = p;
        }
        if let Some(p) = meta.get_parent(&triple.obj) {
            triple.obj = p;
        }
        self.triples.add_data_triple(&triple);
    }

    pub fn split(&mut self, node: &u32, p: &u32, meta: &Meta) {
        for t in self.triples.data_triples {
            self.split_triple(&mut t, node, p, meta);
        }
        for t in self.triples.type_triples {
            self.split_triple(&mut t, node, p, meta);
        }
        self.dict.remove_from_name(p, node);
    }

    pub fn split_triple(&self, triple: &mut Triple, node: &u32, p: &u32, meta: &Meta) {
        if triple.sub == *p {
            if !meta.has_outgoing_pred(node, &triple.pred) {
                return;
            }
            if !meta.has_outgoing_pred(p, &triple.pred) {
                triple.sub = *node;
            } else {
                let new = triple.clone();
                new.sub = *node;
                self.triples.add_data_triple(&new);
            }
        } else if triple.obj == *p {
            if !meta.has_incoming_pred(node, &triple.pred) {
                return;
            }
            if !meta.has_incoming_pred(p, &triple.pred) {
                triple.obj = *node;
            } else {
                let new = triple.clone();
                new.obj = *node;
                self.triples.add_data_triple(&new);
            }
        }
    }

    pub fn to_single_node(&mut self, node: &u32, p: &u32) {
        for t in self.triples.data_triples.iter_mut() {
            t.rename_node(&p, &node);
        }
        for t in self.triples.type_triples.iter_mut() {
            t.rename_node(&p, &node);
        }
        self.dict.remove_by_value(p);
    }
}
