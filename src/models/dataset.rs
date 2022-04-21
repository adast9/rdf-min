use crate::models::dict::Dict;
use crate::models::triple::TripleCollection;

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
        let mut new_triple = triple.clone();

        if let Some(p) = meta.get_parent(&triple.sub) {
            new_triple.sub = p;
        }
        if let Some(p) = meta.get_parent(&triple.obj) {
            new_triple.obj = p;
        }
        self.triples.add_data_triple(&new_triple);
    }

    pub fn split(&mut self, node: &u32, p: &u32, meta: &Meta, to_single: bool) {
        let mut data_triples_to_remove: Vec<usize> = Vec::new();
        let mut type_triples_to_remove: Vec<usize> = Vec::new();

        for i in 0..self.triples.data_triples.len() {
            self.split_triple(i, node, p, meta, true, &mut data_triples_to_remove);
        }
        for i in 0..self.triples.type_triples.len() {
            self.split_triple(i, node, p, meta, false, &mut type_triples_to_remove);
        }

        for i in data_triples_to_remove.iter().rev() {
            self.triples.data_triples.remove(*i);
        }
        for i in type_triples_to_remove.iter().rev() {
            self.triples.type_triples.remove(*i);
        }

        if !to_single {
            self.dict.remove_from_name(p, node);
        }
    }

    pub fn split_triple(
        &mut self,
        i: usize,
        node: &u32,
        p: &u32,
        meta: &Meta,
        is_data: bool,
        to_remove: &mut Vec<usize>,
    ) {
        let triples = if is_data {
            &mut self.triples.data_triples
        } else {
            &mut self.triples.type_triples
        };

        if triples[i].sub == *p {
            if !meta.has_outgoing_triple(node, &triples[i].pred, &triples[i].obj) {
                return;
            }
            if !meta.has_outgoing_triple(p, &triples[i].pred, &triples[i].obj) {
                if !triples.contains(&Triple::new(
                    *node,
                    triples[i].pred,
                    triples[i].obj,
                    triples[i].is_type,
                )) {
                    triples[i].sub = *node;
                } else {
                    to_remove.push(i);
                }
            } else {
                let mut new = triples[i].clone();
                new.sub = *node;
                self.triples.add_data_triple(&new);
            }
        } else if triples[i].obj == *p {
            if !meta.has_incoming_triple(&triples[i].sub, &triples[i].pred, node) {
                return;
            }
            if !meta.has_incoming_triple(&triples[i].sub, &triples[i].pred, p) {
                if !triples.contains(&Triple::new(
                    triples[i].sub,
                    triples[i].pred,
                    *node,
                    triples[i].is_type,
                )) {
                    triples[i].obj = *node;
                } else {
                    to_remove.push(i);
                }

                triples[i].obj = *node;
            } else {
                let mut new = triples[i].clone();
                new.obj = *node;
                self.triples.add_data_triple(&new);
            }
        }
    }

    pub fn to_single_node(&mut self, p: &u32, node: &u32) {
        for t in self.triples.data_triples.iter_mut() {
            t.rename_node(&p, &node);
        }
        for t in self.triples.type_triples.iter_mut() {
            t.rename_node(&p, &node);
        }
        if self.dict.contains_value(p) {
            self.dict.remove_by_value(p);
        }
    }

    /// Removes all nodes in `snode` and inserts `new_node`.
    pub fn new_snode(&mut self, snode: &Vec<u32>, meta: &Meta) -> u32 {
        let mut remove: Vec<u32> = Vec::new();
        let mut snode_string = self.dict.key_by_value(&snode[0]).unwrap();
        snode_string = remove_angle_bracket_at_end(&snode_string).to_string();
        if meta.contains_supernode(&snode[0]) {
            remove.push(snode[0]);
        }

        for node in snode.iter().skip(1) {
            let node_string = self.dict.key_by_value(node).unwrap();
            snode_string.push_str("_");
            snode_string.push_str(&Dict::get_name(&node_string));
            if meta.contains_supernode(&node) {
                remove.push(*node);
            }
        }

        snode_string.push_str(">");
        let id = self.dict.add(&snode_string);
        self.rename_triples(snode, &id);

        for r in remove {
            self.dict.remove_by_value(&r);
        }

        return id;
    }

    /// Replaces all occurences of a node in `snode` with `new_node` in `triples`.
    fn rename_triples(&mut self, old: &Vec<u32>, new: &u32) {
        let mut data_triples_to_remove: Vec<usize> = Vec::new();
        let mut type_triples_to_remove: Vec<usize> = Vec::new();

        for i in 0..self.triples.data_triples.len() {
            for n in old {
                Dataset::rename_node(
                    &mut self.triples.data_triples,
                    i,
                    n,
                    new,
                    &mut data_triples_to_remove,
                );
            }
        }
        for i in 0..self.triples.type_triples.len() {
            for n in old {
                Dataset::rename_node(
                    &mut self.triples.type_triples,
                    i,
                    n,
                    new,
                    &mut type_triples_to_remove,
                );
            }
        }

        for i in data_triples_to_remove.iter().rev() {
            self.triples.data_triples.remove(*i);
        }
        for i in type_triples_to_remove.iter().rev() {
            self.triples.type_triples.remove(*i);
        }
    }

    pub fn key_by_value(&self, value: &u32) -> Option<String> {
        return self.dict.key_by_value(value);
    }

    pub fn dict_strings(&self) -> Vec<String> {
        return self.dict.to_strings();
    }

    fn rename_node(
        triples: &mut Vec<Triple>,
        i: usize,
        old: &u32,
        new: &u32,
        to_remove: &mut Vec<usize>,
    ) {
        if triples[i].sub == *old {
            if !triples.contains(&Triple::new(
                *new,
                triples[i].pred,
                triples[i].obj,
                triples[i].is_type,
            )) {
                triples[i].sub = *new;
            } else {
                to_remove.push(i);
            }
        }
        if triples[i].obj == *old {
            if !triples.contains(&Triple::new(
                triples[i].sub,
                triples[i].pred,
                *new,
                triples[i].is_type,
            )) {
                triples[i].obj = *new;
            } else {
                to_remove.push(i);
            }
        }
    }
}

fn remove_angle_bracket_at_end(string: &String) -> &str {
    let mut chars = string.chars();
    chars.next_back();

    return chars.as_str();
}
