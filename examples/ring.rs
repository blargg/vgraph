use vgraph::{VGraph, a_star_search};


/// Holds the tiles that are used for this puzzle.
/// Tiles loop around in a ring.
struct RingPuzzle {
    spaces: Vec<i32>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct State {
    position: usize,
    sum: i32,
}

impl RingPuzzle {
    fn new(spaces: Vec<i32>) -> Self {
        RingPuzzle {
            spaces,
        }
    }
}

impl VGraph for RingPuzzle {
    type Node = State;

    type Dist = usize;

    fn out_edges(&self, node: Self::Node) -> Vec<Self::Node> {
        // Negative sums end the game. No outgoing states.
        if node.sum < 0 {
            return vec![]
        }

        let left = {
            let pos = if node.position == 0 {
                self.spaces.len() - 1
            } else {
                node.position - 1
            };

            State { position: pos, sum: node.sum + self.spaces[pos] }
        };
        let right = {
            let pos = (node.position + 1) % self.spaces.len();
            State { position: pos, sum: node.sum + self.spaces[pos]}
        };

        vec![left, right]
    }

    fn dist(&self, _from: Self::Node, _to: Self::Node) -> Self::Dist {
        // Each move costs 1 (each turn taken).
        1
    }
}

/// Implementation of the ring puzzle.
fn main() {
    let puzzle = RingPuzzle::new(vec![-3, 7, -9, 4, -8, 1]);
    let start = State { position: 0, sum: 10 };
    let solution = a_star_search(puzzle, start, |s| s.sum == 0, |_| 0);
    println!("Ring 1 solution: {solution:?}");

    let ring3 = RingPuzzle::new(vec![-33, 25, 15, -45, 55, 10]);
    let start3 = State { position: 0, sum: 10 };
    let solution = a_star_search(ring3, start3, |s| s.sum == 0, |_| 0);
    println!("Ring 3 solution: {solution:?}");
}