use std::collections::HashMap;

use crate::parser::{clique::Clique, meta::NodeInfo, triple::Triple};

use super::{do_intersection, intersects};

pub fn delete_triple(
    deletions: &Vec<Triple>,
    source_clique: &mut Vec<Clique>,
    target_clique: &mut Vec<Clique>,
    nodes: HashMap<u32, NodeInfo>,
    supernodes: HashMap<u32, Vec<u32>>,
) {
    for d in deletions {
        let mut list_of_incoming: Vec<Vec<u32>> = Vec::new();
        let mut list_of_outgoing: Vec<Vec<u32>> = Vec::new();
        let p = nodes.get(&d.obj).unwrap().parent;

        if p.is_some() {
            let mut list_of_singlenodes = supernodes.get(&p.unwrap()).unwrap().to_vec();

            for (index, singlenode) in list_of_singlenodes.clone().iter().enumerate() {
                if singlenode == &d.obj {
                    list_of_singlenodes.remove(index);
                }
            }
            for sn in list_of_singlenodes {
                let inc = &nodes.get(&sn).unwrap().incoming;
                let out = &nodes.get(&sn).unwrap().outgoing;
                let mut incoming_preds: Vec<u32> = Vec::new();
                let mut outgoing_preds: Vec<u32> = Vec::new();

                for e in inc {
                    incoming_preds.push(e[0]);
                }
                for e in out {
                    outgoing_preds.push(e[0]);
                }
                list_of_incoming.push(incoming_preds);
                list_of_outgoing.push(outgoing_preds);
            }

            let mut is_intersect_incoming: bool = false;
            for e1 in &nodes.get(&d.obj).unwrap().incoming {
                for e2 in &list_of_incoming {
                    if intersects(e1, e2) {
                        is_intersect_incoming = true;
                        break;
                    }
                }

                if is_intersect_incoming == true {
                    break;
                }
            }
        } else {
            //do somethings!
        }
    }
}
