#[cfg(test)]
mod clique_tests {
    use crate::clique::Clique;

    #[test]
    fn new() {
        let c1 = Clique { preds: vec![1, 2, 3], nodes: vec![4, 5, 6] };
        let c2 = Clique::new(&vec![1, 2, 3], &vec![4, 5, 6]);

        assert_eq!(c1.preds, c2.preds);
        assert_eq!(c1.nodes, c2.nodes);
    }

    #[test]
    fn merge() {
        let mut c1 = Clique::new(&vec![1, 2, 3], &vec![7, 8, 9]);
        let c2 = Clique::new(&vec![4, 5, 6], &vec![10, 11, 12]);

        c1.merge(&c2);
        c1.preds.sort();
        c1.nodes.sort();

        assert_eq!(c1.preds, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(c1.nodes, vec![7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn node_intersection() {
        let c1 = Clique::new(&vec![], &vec![1, 2, 3]);
        let c2 = Clique::new(&vec![], &vec![2, 3, 4]);

        let mut c3 = c1.node_intersection(&c2);
        c3.sort();

        assert_eq!(c3, vec![2, 3]);
    }
}