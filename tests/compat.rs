//! Value-exact compatibility with networkx 3.6.1.
//!
//! Each row is `(edge-list golden, [bipartite, eulerian, eulerian-path, tree,
//! forest, regular])` where the six booleans were captured from networkx 3.6.1
//! (`nx.is_bipartite`, `nx.is_eulerian`, `nx.has_eulerian_path`, `nx.is_tree`,
//! `nx.is_forest`, `nx.is_regular`) on the graph built from that same edge list
//! (`nx.Graph().add_edges_from(...)`). No Python runs at test time — the oracle
//! values are frozen constants below.

use rsomics_graph_properties::{parse_edge_list, Property};

const PROPERTIES: [Property; 6] = [
    Property::Bipartite,
    Property::Eulerian,
    Property::EulerianPath,
    Property::Tree,
    Property::Forest,
    Property::Regular,
];

struct Case {
    name: &'static str,
    edges: &'static str,
    // [bipartite, eulerian, eulerian-path, tree, forest, regular]
    expected: [bool; 6],
}

const CASES: &[Case] = &[
    Case {
        name: "c4",
        edges: include_str!("golden/c4.txt"),
        expected: [true, true, true, false, false, true],
    },
    Case {
        name: "c5",
        edges: include_str!("golden/c5.txt"),
        expected: [false, true, true, false, false, true],
    },
    Case {
        name: "path4",
        edges: include_str!("golden/path4.txt"),
        expected: [true, false, true, true, true, false],
    },
    Case {
        name: "triangle",
        edges: include_str!("golden/triangle.txt"),
        expected: [false, true, true, false, false, true],
    },
    Case {
        name: "star5",
        edges: include_str!("golden/star5.txt"),
        expected: [true, false, false, true, true, false],
    },
    Case {
        name: "two_paths",
        edges: include_str!("golden/two_paths.txt"),
        expected: [true, false, false, false, true, false],
    },
    Case {
        name: "cycle_tail",
        edges: include_str!("golden/cycle_tail.txt"),
        expected: [false, false, true, false, false, false],
    },
    Case {
        name: "two_edges",
        edges: include_str!("golden/two_edges.txt"),
        expected: [true, false, false, false, true, true],
    },
    Case {
        name: "karate",
        edges: include_str!("golden/karate.txt"),
        expected: [false, false, false, false, false, false],
    },
    Case {
        name: "gnm_30_22_s1",
        edges: include_str!("golden/gnm_30_22_s1.txt"),
        expected: [true, false, false, false, false, false],
    },
    Case {
        name: "gnm_60_90_s7",
        edges: include_str!("golden/gnm_60_90_s7.txt"),
        expected: [false, false, false, false, false, false],
    },
    Case {
        name: "selfloop_only",
        edges: include_str!("golden/selfloop_only.txt"),
        expected: [false, true, true, false, false, true],
    },
    Case {
        name: "selfloop_edge",
        edges: include_str!("golden/selfloop_edge.txt"),
        expected: [false, false, true, false, false, false],
    },
    Case {
        name: "triangle_selfloop",
        edges: include_str!("golden/triangle_selfloop.txt"),
        expected: [false, true, true, false, false, false],
    },
];

#[test]
fn all_properties_match_networkx() {
    for case in CASES {
        let g = parse_edge_list(case.edges);
        for (prop, &want) in PROPERTIES.iter().zip(case.expected.iter()) {
            let got = prop.evaluate(&g);
            assert_eq!(
                got,
                want,
                "graph {} property {}: got {got}, networkx says {want}",
                case.name,
                prop.as_str(),
            );
        }
    }
}
