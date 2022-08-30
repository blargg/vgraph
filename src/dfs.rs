use crate::VGraph;


/// Iterates over all paths from start to end node set.
/// Uses DFS.
struct PathsIter<G, F>
where
    G: VGraph,
    F: Fn(G::Node) -> bool,
{
    g: G,
    start: G::Node,
    is_end: F,
}

impl <G, F> Iterator for PathsIter<G, F>
where
    G: VGraph,
    F: Fn(G::Node) -> bool,
{
    type Item = Vec<G::Node>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// TODO: modify this to search paths matching a filter
pub fn dfs<G, F>(g: G, start: G::Node, is_end: F) -> impl Iterator<Item=Vec<G::Node>>
where
    G: VGraph,
    F: Fn(G::Node) -> bool,
{
    let cur_path = vec![start];
    PathsIter {
        g, start, is_end
    }
}