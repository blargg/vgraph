use num::traits::Zero;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::AddAssign;

/// Virtual Graph.
pub trait VGraph {
    type Node;

    // Measure of the distance between nodes
    type Dist;

    fn out_edges(&self, node: Self::Node) -> Vec<Self::Node>;

    fn dist(&self, from: Self::Node, to: Self::Node) -> Self::Dist;
}

pub fn breadth_first_search<G>(g: G, start: G::Node, end: G::Node) -> Option<Vec<G::Node>>
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

        for next in g.out_edges(cur) {
            // Only insert nodes we have not explored yet
            if !prev.contains_key(&next) {
                prev.insert(next, cur);
                to_explore.push_back(next);
            }
        }
    }

    None
}

pub fn a_star_search<G, F, H>(g:G, start: G::Node, is_end: F, heuristic: H) -> Option<Vec<G::Node>>
where
    G: VGraph,
    G::Node: Hash + Eq + Ord + Copy,
    G::Dist: Zero + Ord + Copy,
    F: Fn(G::Node) -> bool,
    H: Fn(G::Node) -> G::Dist,
{
    let mut to_explore = PriorityQueue::new();
    to_explore.push_increase(start, Reverse(heuristic(start)));
    // Stores the node that this came from on the path, and the best found true distance from the start.
    let mut prev: HashMap<G::Node, G::Node> = HashMap::new();
    let mut dist_from_start: HashMap<G::Node, G::Dist> = HashMap::new();
    dist_from_start.insert(start, G::Dist::zero());
    while let Some((cur, _priority)) = to_explore.pop() {
        if is_end(cur) {
            return Some(back_track(&prev, cur));
        }

        for next in g.out_edges(cur) {
            let cur_distance = dist_from_start
                .get(&cur)
                .expect("Every node in the explore set should already have a previous distance.");
            let start_to_next: G::Dist = *cur_distance + g.dist(cur, next);
            let h_dist = start_to_next + heuristic(next);

            if let Some(best_start_to_next) = dist_from_start.get(&next) {
                // we already have a path to next that is better than this one, skip this path.
                if start_to_next >= *best_start_to_next { continue; }
            }
            to_explore.push_increase(next, Reverse(h_dist));

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

pub fn path_length<G>(g: G, path: Vec<G::Node>) -> G::Dist
where
    G: VGraph,
    G::Node: Copy,
    G::Dist: Copy + AddAssign + Zero,
{
    let mut distance = G::Dist::zero();
    for window in path.windows(2) {
        if let &[from, to] = window {
            distance += g.dist(from, to);
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
    impl Ex {
        fn new() -> Self {
            Ex {}
        }
    }

    impl VGraph for Ex {
        type Node = usize;

        fn out_edges(&self, node: Self::Node) -> Vec<Self::Node> {
            match node {
                1 => vec![2],
                2 => vec![3],
                3 => vec![],
                _ => vec![],
            }
        }

        type Dist = i32;

        // If a node is in the graph, then the distance is 1
        fn dist(&self, _: Self::Node, _: Self::Node) -> Self::Dist {
            1
        }
    }

    struct Cycles {}

    impl VGraph for Cycles {
        type Node = usize;

        type Dist = usize;

        fn out_edges(&self, node: Self::Node) -> Vec<Self::Node> {
            match node {
                1 => vec![2,3],
                2 => vec![3,6],
                3 => vec![4, 5],
                4 => vec![10, 5],
                5 => vec![1],
                6 => vec![2],
                7 => vec![8],
                8 => vec![9],
                9 => vec![10],
                10 => vec![1],
                _ => vec![],
            }
        }

        fn dist(&self, from: Self::Node, _to: Self::Node) -> Self::Dist {
            match from {
                3 => 3,
                _ => 1,
            }
        }
    }

    #[test]
    fn breadth_first_search_works() {
        assert_eq!(Some(vec![1, 2, 3]), breadth_first_search(Ex::new(), 1, 3));
    }

    #[test]
    fn breadth_first_search_for_non_path_terminates() {
        assert_eq!(None, breadth_first_search(Cycles {}, 1, 33));
    }

    #[test]
    fn path_length_works() {
        assert_eq!(2, path_length::<Ex>(Ex::new(), vec![1, 2, 3]))
    }

    #[test]
    fn a_star_search_works() {
        // This ignores the heuristic and makes this equivalent to djikstra's
        fn h(_node: usize) -> i32 {
            return 0;
        }
        assert_eq!(Some(vec![1, 2, 3]), a_star_search(Ex::new(), 1, |n| n == 3, h));
    }

    #[test]
    fn a_star_search_for_non_path_terminates() {
        assert_eq!(None, a_star_search(Cycles {}, 1, |n| n == 33, |_| 0));
    }
}
