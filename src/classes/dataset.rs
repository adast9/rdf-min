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

    /// Removes all nodes in `snode` and inserts `new_node`.
    pub fn new_snode(self, snode: &Vec<u32>) -> u32 {
        let mut snode_string = self.dict.key_by_value(&snode[0]).unwrap();
        snode_string = remove_angle_bracket_at_end(&snode_string).to_string();

        for node in snode.iter().skip(1) {
            let node_string = self.dict.key_by_value(node).unwrap();
            snode_string.push_str("_");
            snode_string.push_str(&Dict::get_name(&node_string));
        }

        snode_string.push_str(">");
        let id = self.dict.add(&snode_string);
        self.rename_triples(snode, &id);
        return id;
    }

    /// Replaces all occurences of a node in `snode` with `new_node` in `triples`.
    fn rename_triples(&mut self, old: &Vec<u32>, new: &u32) {
        for t in self.triples.data_triples {
            for n in old {
                t.rename_node(&n, new);
            }
        }
        for t in self.triples.type_triples {
            for n in old {
                t.rename_node(&n, new);
            }
        }
    }
}

fn remove_angle_bracket_at_end(string: &String) -> &str {
    let mut chars = string.chars();
    chars.next_back();

    return chars.as_str();
}
