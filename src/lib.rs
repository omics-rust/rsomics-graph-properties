//! Undirected graph-property predicates — value-exact port of a battery of
//! `networkx` boolean graph invariants (`is_bipartite`, `is_eulerian`,
//! `has_eulerian_path`, `is_tree`, `is_forest`, `is_regular`).
//!
//! Each predicate is a deterministic integer/combinatorial invariant of the
//! graph, so "value-exact" means the exact same `true`/`false` networkx
//! returns — no floats, no tolerance.
//!
//! # Graph contract
//!
//! Input is an undirected edge list. The graph is the *simple* graph over the
//! **node set induced by the edge list**: parallel edges are deduplicated but
//! self-loops are kept, exactly as constructing an `nx.Graph` from that edge
//! list. A self-loop counts as one edge and adds 2 to its node's degree, and it
//! is a length-1 odd cycle (so the graph is not bipartite). An edge list cannot
//! express isolated (degree-0) nodes, so those are unrepresentable here;
//! networkx's special handling of isolated vertices therefore never applies to
//! a graph parsed from an edge list.

use std::collections::HashMap;

/// Undirected simple graph over interned integer node ids `0..n`.
///
/// `adj[v]` holds neighbours excluding self; `self_loop[v]` records whether the
/// node carries a self-loop. Degree is `adj[v].len() + 2` when it does, matching
/// networkx.
pub struct Graph {
    idx_to_node: Vec<String>,
    adj: Vec<Vec<usize>>,
    self_loop: Vec<bool>,
}

impl Graph {
    fn intern(&mut self, name: &str, table: &mut HashMap<String, usize>) -> usize {
        if let Some(&idx) = table.get(name) {
            return idx;
        }
        let idx = self.idx_to_node.len();
        table.insert(name.to_owned(), idx);
        self.idx_to_node.push(name.to_owned());
        self.adj.push(Vec::new());
        self.self_loop.push(false);
        idx
    }

    #[must_use]
    pub fn number_of_nodes(&self) -> usize {
        self.idx_to_node.len()
    }

    #[must_use]
    pub fn number_of_edges(&self) -> usize {
        self.adj.iter().map(Vec::len).sum::<usize>() / 2
            + self.self_loop.iter().filter(|&&s| s).count()
    }

    fn degrees(&self) -> Vec<usize> {
        (0..self.number_of_nodes())
            .map(|v| self.adj[v].len() + if self.self_loop[v] { 2 } else { 0 })
            .collect()
    }
}

/// Parse a whitespace-delimited `u v` edge list. `#` comments and blank lines
/// are skipped. Parallel edges are deduplicated; self-loops are kept, giving the
/// undirected simple graph (with self-loops) over the edge-list node set.
#[must_use]
pub fn parse_edge_list(input: &str) -> Graph {
    let mut g = Graph {
        idx_to_node: Vec::new(),
        adj: Vec::new(),
        self_loop: Vec::new(),
    };
    let mut table = HashMap::new();

    for line in input.lines() {
        // nx.parse_edgelist strips a '#' comment anywhere in the line before tokenising.
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let (Some(u), Some(v)) = (parts.next(), parts.next()) else {
            continue;
        };
        let ui = g.intern(u, &mut table);
        let vi = g.intern(v, &mut table);
        if ui == vi {
            g.self_loop[ui] = true;
            continue;
        }
        if !g.adj[ui].contains(&vi) {
            g.adj[ui].push(vi);
            g.adj[vi].push(ui);
        }
    }
    g
}

/// The six supported properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    Bipartite,
    Eulerian,
    EulerianPath,
    Tree,
    Forest,
    Regular,
}

impl Property {
    /// The `--property` token used on the CLI and reported in `--json`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Property::Bipartite => "bipartite",
            Property::Eulerian => "eulerian",
            Property::EulerianPath => "eulerian-path",
            Property::Tree => "tree",
            Property::Forest => "forest",
            Property::Regular => "regular",
        }
    }

    /// Evaluate this property on a parsed graph. Compute-only: no parsing.
    #[must_use]
    pub fn evaluate(self, g: &Graph) -> bool {
        match self {
            Property::Bipartite => is_bipartite(g),
            Property::Eulerian => is_eulerian(g),
            Property::EulerianPath => has_eulerian_path(g),
            Property::Tree => is_tree(g),
            Property::Forest => is_forest(g),
            Property::Regular => is_regular(g),
        }
    }
}

/// Number of connected components. Used by `is_forest` (`edges == nodes - k`).
fn count_components(g: &Graph) -> usize {
    let n = g.number_of_nodes();
    let mut seen = vec![false; n];
    let mut stack = Vec::new();
    let mut comps = 0;
    for start in 0..n {
        if seen[start] {
            continue;
        }
        comps += 1;
        seen[start] = true;
        stack.push(start);
        while let Some(v) = stack.pop() {
            for &w in &g.adj[v] {
                if !seen[w] {
                    seen[w] = true;
                    stack.push(w);
                }
            }
        }
    }
    comps
}

/// True iff the graph is connected (single component over all its nodes).
fn is_connected(g: &Graph) -> bool {
    g.number_of_nodes() > 0 && count_components(g) == 1
}

/// `nx.is_bipartite`: 2-colorable via BFS coloring. False on any odd cycle.
#[must_use]
pub fn is_bipartite(g: &Graph) -> bool {
    if g.self_loop.iter().any(|&s| s) {
        return false;
    }
    let n = g.number_of_nodes();
    let mut color = vec![-1i8; n];
    let mut queue = std::collections::VecDeque::new();
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        color[start] = 0;
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            let cv = color[v];
            for &w in &g.adj[v] {
                if color[w] == -1 {
                    color[w] = 1 - cv;
                    queue.push_back(w);
                } else if color[w] == cv {
                    return false;
                }
            }
        }
    }
    true
}

/// `nx.is_eulerian` (undirected): every vertex has even degree AND the graph
/// is connected.
#[must_use]
pub fn is_eulerian(g: &Graph) -> bool {
    g.degrees().iter().all(|&d| d.is_multiple_of(2)) && is_connected(g)
}

/// `nx.has_eulerian_path` (undirected): eulerian, OR exactly two vertices of
/// odd degree AND connected.
#[must_use]
pub fn has_eulerian_path(g: &Graph) -> bool {
    if is_eulerian(g) {
        return true;
    }
    let odd = g.degrees().iter().filter(|&&d| d % 2 == 1).count();
    odd == 2 && is_connected(g)
}

/// `nx.is_tree`: connected AND `edges == nodes - 1`. networkx raises on an
/// empty graph; an edge-list-parsed graph always has ≥1 node, so we return
/// `false` for the (unreachable-from-edge-list) empty case rather than panic.
#[must_use]
pub fn is_tree(g: &Graph) -> bool {
    let n = g.number_of_nodes();
    n > 0 && g.number_of_edges() == n - 1 && is_connected(g)
}

/// `nx.is_forest`: every connected component is a tree, i.e. globally
/// `edges == nodes - num_components`.
#[must_use]
pub fn is_forest(g: &Graph) -> bool {
    let n = g.number_of_nodes();
    n > 0 && g.number_of_edges() == n - count_components(g)
}

/// `nx.is_regular`: all nodes have the same degree.
#[must_use]
pub fn is_regular(g: &Graph) -> bool {
    let degrees = g.degrees();
    match degrees.first() {
        Some(&d0) => degrees.iter().all(|&d| d == d0),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint(g: &Graph) -> (usize, usize, bool, bool, bool, bool, bool) {
        (
            g.number_of_nodes(),
            g.number_of_edges(),
            is_bipartite(g),
            is_eulerian(g),
            has_eulerian_path(g),
            is_tree(g),
            is_regular(g),
        )
    }

    #[test]
    fn inline_hash_comment_matches_comment_free_graph() {
        let with_comments = parse_edge_list("0 1\n1 2#c\n2 3\n0 #x\n");
        let clean = parse_edge_list("0 1\n1 2\n2 3\n");
        assert_eq!(fingerprint(&with_comments), fingerprint(&clean));
        assert_eq!(with_comments.number_of_nodes(), 4);
        assert_eq!(with_comments.number_of_edges(), 3);
    }
}
