use crate::parser::meta::{MetaFile, Node, Supernode};
use std::collections::HashMap;

use super::triple::Triple;

pub struct Meta {
    supernodes: HashMap<u32, Vec<u32>>,
    nodes: HashMap<u32, NodeInfo>,
    types: HashMap<u32, u32>,
}

impl Meta {
    pub fn new(
        supernodes: HashMap<u32, Vec<u32>>,
        nodes: HashMap<u32, NodeInfo>,
        types: HashMap<u32, u32>,
    ) -> Self {
        Self {
            supernodes,
            nodes,
            types,
        }
    }

    pub fn serialize(&self) -> MetaFile {
        let mut s: Vec<Supernode> = Vec::new();
        let mut q: Vec<Node> = Vec::new();
        let mut t: Vec<[u32; 2]> = Vec::new();

        for (k, v) in &self.supernodes {
            s.push(Supernode {
                i: *k,
                g: v.to_vec(),
            });
        }

        for (k, v) in &self.nodes {
            q.push(Node {
                i: *k,
                p: v.parent,
                n: v.incoming.to_vec(),
                o: v.outgoing.to_vec(),
            });
        }

        for (k, v) in &self.types {
            t.push([*k, *v]);
        }

        return MetaFile { s, q, t };
    }

    pub fn deserialize(file: MetaFile) -> Self {
        let mut supernodes: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut nodes: HashMap<u32, NodeInfo> = HashMap::new();
        let mut types: HashMap<u32, u32> = HashMap::new();

        for snode in file.s {
            supernodes.insert(snode.i, snode.g.to_vec());
        }

        for node in file.q {
            nodes.insert(node.i, NodeInfo::new(&node.p, &node.n, &node.o));
        }

        for ty in file.t {
            types.insert(ty[0], ty[1]);
        }

        return Self::new(supernodes, nodes, types);
    }

    pub fn contains(&self, node: &u32) -> bool {
        return self.nodes.contains_key(&node) || self.supernodes.contains_key(&node);
    }

    pub fn contains_supernode(&self, node: &u32) -> bool {
        return self.supernodes.contains_key(&node);
    }

    pub fn new_node(&mut self, triple: &Triple, is_sub: bool) {
        let node = if is_sub { triple.sub } else { triple.obj };
        let other = if is_sub { triple.obj } else { triple.sub };
        if self.contains(&node) {
            panic!("Trying to add new node {}, but it already exists", node);
        }
        self.nodes.insert(
            node,
            if !is_sub {
                NodeInfo::new(&None, &vec![[triple.pred, other]], &vec![])
            } else {
                NodeInfo::new(&None, &vec![], &vec![[triple.pred, other]])
            },
        );
    }

    pub fn add_outgoing(&mut self, triple: &Triple) {
        self.nodes
            .get_mut(&triple.sub)
            .unwrap()
            .outgoing
            .push([triple.pred, triple.obj]);
    }

    pub fn add_incoming(&mut self, triple: &Triple) {
        self.nodes
            .get_mut(&triple.obj)
            .unwrap()
            .incoming
            .push([triple.pred, triple.sub]);
    }

    pub fn remove_outgoing(&mut self, triple: &Triple) {
        self.nodes
            .get_mut(&triple.sub)
            .unwrap()
            .outgoing
            .retain(|x| !(x[0] == triple.pred && x[1] == triple.obj));
    }

    pub fn remove_incoming(&mut self, triple: &Triple) {
        self.nodes
            .get_mut(&triple.obj)
            .unwrap()
            .incoming
            .retain(|x| !(x[0] == triple.pred && x[1] == triple.sub));
    }

    pub fn get_parent(&self, node: &u32) -> Option<u32> {
        return self.nodes.get(node).unwrap().parent;
    }

    pub fn has_parent(&self, node: &u32) -> bool {
        return self.get_parent(node).is_some();
    }

    pub fn remove_from_supernode(&mut self, node: &u32) -> bool {
        let p = self.get_parent(node).unwrap();
        self.supernodes.get_mut(&p).unwrap().retain(|x| *x != *node);
        self.nodes.get_mut(node).unwrap().remove_parent();
        if self.supernode_len(&p) == 1 {
            return true;
        }
        return false;
    }

    pub fn has_incoming_triple(&self, s: &u32, p: &u32, o: &u32) -> bool {
        if !self.contains_supernode(o) {
            for v in &self.nodes.get(o).unwrap().incoming {
                if v[0] == *p {
                    if let Some(parent) = self.get_parent(&v[1]) {
                        if parent == *s {
                            return true;
                        }
                    } else if v[1] == *s {
                        return true;
                    }
                }
            }
            return false;
        } else {
            for v in self.supernodes.get(o).unwrap() {
                if self.has_incoming_triple(s, p, v) {
                    return true;
                }
            }
            return false;
        }
    }

    pub fn has_outgoing_triple(&self, s: &u32, p: &u32, o: &u32) -> bool {
        if !self.contains_supernode(s) {
            for v in &self.nodes.get(s).unwrap().outgoing {
                if v[0] == *p {
                    if v[1] == *o {
                        return true;
                    }
                    if let Some(parent) = self.get_parent(&v[1]) {
                        if parent == *o {
                            return true;
                        }
                    }
                }
            }
            return false;
        } else {
            for v in self.supernodes.get(s).unwrap() {
                if self.has_outgoing_triple(v, p, o) {
                    return true;
                }
            }
            return false;
        }
    }

    pub fn supernode_len(&self, node: &u32) -> usize {
        if !self.contains_supernode(node) {
            panic!("Trying to get length of non-supernode {:?}", node);
        }
        return self.supernodes.get(node).unwrap().len();
    }

    pub fn to_single_node(&mut self, snode: &u32) {
        if !self.contains_supernode(snode) {
            panic!("Trying to convert non-supernode {:?} to single node", snode);
        } else if !self.supernode_len(snode) == 1 {
            panic!(
                "Trying to convert supernode {:?} to single node, but it has more than one node",
                snode
            );
        }
        let node = self.supernodes.get(snode).unwrap()[0];
        self.nodes.get_mut(&node).unwrap().remove_parent();
        self.supernodes.remove(snode);
    }

    /// Combines all nodes in `snode` into a single supernode in `stuff.supernodes`.
    /// Also updates the `parent` field of all nodes in `snode`.
    pub fn new_snode(&mut self, old: &Vec<u32>, new: &u32) {
        let mut new_snode: Vec<u32> = Vec::new();

        for n in old {
            if self.contains_supernode(&n) {
                let sn = self.supernodes.get(n).unwrap();
                new_snode.extend(sn);

                for s in sn {
                    self.nodes.get_mut(s).unwrap().set_parent(new);
                }
                self.supernodes.remove(n);
            } else {
                self.nodes.get_mut(n).unwrap().set_parent(new);
                new_snode.push(*n);
            }
        }
        self.supernodes.insert(*new, new_snode);
    }

    pub fn get_supernode(&self, n: &u32) -> Option<&Vec<u32>> {
        return self.supernodes.get(n);
    }

    pub fn get_mut_supernode(&mut self, n: &u32) -> Option<&mut Vec<u32>> {
        return self.supernodes.get_mut(n);
    }

    pub fn has_no_edges_left(&self, n: &u32, is_source: bool) -> bool {
        if is_source {
            return self.nodes.get(n).unwrap().outgoing.is_empty();
        } else {
            return self.nodes.get(n).unwrap().incoming.is_empty();
        }
    }

    pub fn get_preds(&self, n: &u32, is_source: bool) -> Vec<u32> {
        if is_source {
            return self.get_outgoing_preds(n);
        } else {
            return self.get_incoming_preds(n);
        }
    }

    pub fn get_incoming_preds(&self, n: &u32) -> Vec<u32> {
        let mut preds: Vec<u32> = Vec::new();
        for t in &self.nodes.get(n).unwrap().incoming {
            preds.push(t[0]);
        }
        return preds;
    }

    pub fn get_outgoing_preds(&self, n: &u32) -> Vec<u32> {
        let mut preds: Vec<u32> = Vec::new();
        for t in &self.nodes.get(n).unwrap().outgoing {
            preds.push(t[0]);
        }
        return preds;
    }

    pub fn remove_supernode(&mut self, id: &u32) {
        let sn = self.get_supernode(id).unwrap().clone();
        for n in sn {
            self.nodes.get_mut(&n).unwrap().remove_parent();
        }
        self.supernodes.remove(id);
    }

    pub fn get_types(&self) -> &HashMap<u32, u32> {
        return &self.types;
    }

    pub fn add_type(&mut self, s: &u32, o: &u32) {
        self.types.insert(*s, *o);
    }
}

pub struct NodeInfo {
    pub parent: Option<u32>,
    pub incoming: Vec<[u32; 2]>,
    pub outgoing: Vec<[u32; 2]>,
}

impl NodeInfo {
    pub fn new(parent: &Option<u32>, incoming: &Vec<[u32; 2]>, outgoing: &Vec<[u32; 2]>) -> Self {
        NodeInfo {
            parent: parent.clone(),
            incoming: incoming.clone(),
            outgoing: outgoing.clone(),
        }
    }

    pub fn remove_parent(&mut self) {
        self.parent = None;
    }

    pub fn set_parent(&mut self, parent: &u32) {
        self.parent = Some(*parent);
    }
}
