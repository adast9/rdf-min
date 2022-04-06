use crate::classes::dict::Dict;
use crate::classes::triple::TripleCollection;

pub struct Dataset {
    dict: Dict,
    pub triples: TripleCollection,
    pub insertions: TripleCollection,
    pub deletions: TripleCollection,
}

impl Dataset {
    pub fn new(t_l: Vec<String>, i_l: Vec<String>, d_l: Vec<String>) -> Self {
        let dict = Dict::empty();
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
        let dict = Dict::new(&dict_l);
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
}
