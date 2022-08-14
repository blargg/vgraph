use num::traits::Zero;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::AddAssign;

/// Virtual Graph.
pub trait VGraph {
    type Node;

    // Measure of the distance between nodes
    type Dist;

    fn all_nodes() -> Vec<Self::Node>;
    fn out_edges(node: Self::Node) -> Vec<Self::Node>;

    fn dist(from: Self::Node, to: Self::Node) -> Self::Dist;
}

pub fn breadth_first_search<G>(start: G::Node, end: G::Node) -> Option<Vec<G::Node>>
where
    G: VGraph,
    G::Node: Eq + Hash + Copy,
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

pub fn a_star_search<G, H>(start: G::Node, end: G::Node, heuristic: H) -> Option<Vec<G::Node>>
where
    G: VGraph,
    G::Node: Hash + Eq + Ord + Copy,
    G::Dist: Zero + Ord + Copy,
    H: Fn(G::Node) -> G::Dist,
{
    let mut to_explore = PriorityQueue::new();
    to_explore.push_decrease(start, heuristic(start));
    // Stores the node that this came from on the path, and the best found true distance from the start.
    let mut prev: HashMap<G::Node, G::Node> = HashMap::new();
    let mut dist_from_start: HashMap<G::Node, G::Dist> = HashMap::new();
    dist_from_start.insert(start, G::Dist::zero());
    while let Some((cur, _priority)) = to_explore.pop() {
        if cur == end {
            return Some(back_track(&prev, end));
        }

        for next in G::out_edges(cur) {
            let cur_distance = dist_from_start
                .get(&cur)
                .expect("Every node in the explore set should already have a previous distance.");
            let start_to_next: G::Dist = *cur_distance + G::dist(cur, next);
            let h_dist = start_to_next + heuristic(next);
            to_explore.push_decrease(next, h_dist);

            if dist_from_start
                .get(&next)
                .map(|current_best_start_to_next| start_to_next < *current_best_start_to_next)
                .unwrap_or(true)
            {
                prev.insert(next, cur);
                dist_from_start.insert(next, start_to_next);
            }
        }
    }

    // Ran out of places to explore, end not found.
    None
}

pub fn path_length<G>(path: Vec<G::Node>) -> G::Dist
where
    G: VGraph,
    G::Node: Copy,
    G::Dist: Copy + AddAssign + Zero,
{
    let mut distance = G::Dist::zero();
    for window in path.windows(2) {
        if let &[from, to] = window {
            distance += G::dist(from, to);
        } else {
            panic!("Windows were not full.");
        }
    }

    distance
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
            vec![1, 2, 3]
        }

        fn out_edges(node: Self::Node) -> Vec<Self::Node> {
            match node {
                1 => vec![2],
                2 => vec![3],
                3 => vec![],
                _ => vec![],
            }
        }

        type Dist = i32;

        // If a node is in the graph, then the distance is 1
        fn dist(_: Self::Node, _: Self::Node) -> Self::Dist {
            1
        }
    }

    #[test]
    fn breadth_first_search_works() {
        assert_eq!(Some(vec![1, 2, 3]), breadth_first_search::<Ex>(1, 3));
    }

    #[test]
    fn path_length_works() {
        assert_eq!(2, path_length::<Ex>(vec![1, 2, 3]))
    }

    #[test]
    fn a_star_search_works() {
        // This ignores the heuristic and makes this equivalent to djikstra's
        fn h(_node: usize) -> i32 {
            return 0;
        }
        assert_eq!(Some(vec![1, 2, 3]), a_star_search::<Ex, _>(1, 3, h));
    }
}
