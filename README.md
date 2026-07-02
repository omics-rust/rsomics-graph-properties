# rsomics-graph-properties

Undirected graph-property predicates — a value-exact Rust port of a battery of
[NetworkX](https://networkx.org/) boolean graph invariants. One binary, one
`--property` flag, one boolean out.

```bash
cargo install rsomics-graph-properties
```

## Usage

Reads an undirected edge list from stdin (or a file), one `u v` per line.
Node labels are arbitrary strings; `#` comments and blank lines are ignored.

```bash
printf '0 1\n1 2\n2 3\n3 0\n' | rsomics-graph-properties --property bipartite
# true

printf '0 1\n1 2\n2 3\n3 0\n' | rsomics-graph-properties --property eulerian
# true

printf '0 1\n1 2\n2 3\n' | rsomics-graph-properties --property tree
# true
```

`--property` (default `bipartite`):

| value | networkx equivalent | true when |
|---|---|---|
| `bipartite` | `nx.is_bipartite` | 2-colorable (no odd cycle) |
| `eulerian` | `nx.is_eulerian` | connected and every vertex has even degree |
| `eulerian-path` | `nx.has_eulerian_path` | connected and exactly 0 or 2 vertices of odd degree |
| `tree` | `nx.is_tree` | connected and `edges == nodes - 1` |
| `forest` | `nx.is_forest` | acyclic: `edges == nodes - components` |
| `regular` | `nx.is_regular` | all vertices have equal degree |

Output is `true` or `false`. With `--json`:

```json
{"property": "bipartite", "value": true}
```

(wrapped in the standard rsomics `--json` envelope).

## Graph contract

The graph is the **simple graph over the node set induced by the edge list**:

- Parallel edges are deduplicated.
- Self-loops are kept, exactly as an `nx.Graph` keeps them: a self-loop counts
  as one edge and adds 2 to its node's degree, and it is a length-1 odd cycle
  (so the graph is not bipartite).
- **Isolated (degree-0) nodes are unrepresentable** — an edge list can only
  name nodes that appear on an edge. NetworkX's special handling of isolated
  vertices (e.g. `is_eulerian`/`has_eulerian_path` returning `False` because an
  isolated vertex breaks connectivity) therefore never arises from edge-list
  input.

For every graph expressible as a plain edge list, the six predicates return the
exact same boolean NetworkX 3.6.1 returns for `nx.Graph().add_edges_from(...)`.

## Origin

This crate is an independent Rust reimplementation of NetworkX's undirected
graph-property predicates, based on:

- The NetworkX 3.6.1 source for `is_bipartite`, `is_eulerian`,
  `has_eulerian_path`, `is_tree`, `is_forest`, and `is_regular`
  (BSD-3-Clause — reading and citing permitted).
- Black-box behavior testing against the NetworkX binary: golden graphs are
  built with NetworkX and each of the six property booleans is captured and
  frozen into `tests/compat.rs`.

Predicates are integer/combinatorial graph invariants (BFS 2-coloring, degree
parity, edge/node/component counts) — value-exact means the exact same
`true`/`false`, with no floating point and no tolerance.

License: MIT OR Apache-2.0.
Upstream credit: [NetworkX](https://github.com/networkx/networkx) (BSD-3-Clause).
