use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Virtual Graph.
pub trait VGraph {
    type Node;

    fn all_nodes() -> Vec<Self::Node>;
    fn out_edges(node: Self::Node) -> Vec<Self::Node>;
}

pub fn breadth_first_search<N, G>(start: G::Node, end: G::Node) -> Option<Vec<G::Node>>
where
    N: Eq + Hash + Copy,
    G: VGraph<Node = N>,
{
    let mut to_explore = VecDeque::new();
    to_explore.push_back(start);
    let mut prev = HashMap::<G::Node, G::Node>::new();

    while let Some(cur) = to_explore.pop_front() {
        if cur == end {
            return Some(back_track(&prev, end));
        }


        for next in G::out_edges(cur) {
            // Only insert nodes we have not explored yet
            if !prev.contains_key(&next) {
                prev.insert(next, cur);
                to_explore.push_back(next);
            }
        }
    }

    None
}

/// Given a prev map, where each index points to the previous value.
/// Returns them in some order.
fn back_track<A: Copy + Eq + Hash>(prev: &HashMap<A, A>, end: A) -> Vec<A> {
    let mut path = Vec::new();
    let mut cur = end;
    while let Some(next) = prev.get(&cur) {
        path.push(cur);
        cur = *next;
    }
    path.push(cur);
    path.reverse();

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ex {}

    impl VGraph for Ex {
        type Node = usize;

        fn all_nodes() -> Vec<Self::Node> {
            vec![1,2,3]
        }

        fn out_edges(node: Self::Node) -> Vec<Self::Node> {
            match node {
                1 => vec![2],
                2 => vec![3],
                3 => vec![],
                _ => vec![],
            }
        }
    }

    #[test]
    fn breadth_first_search_works() {
        assert_eq!(Some(vec![1,2,3]), breadth_first_search::<usize,Ex>(1,3));
    }
}
