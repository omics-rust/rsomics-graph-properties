use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use rsomics_graph_properties::{is_bipartite, is_forest, parse_edge_list};

const GNM: &str = include_str!("../tests/golden/gnm_2000_8000_s1.txt");

fn bench(c: &mut Criterion) {
    // Parse once; time the compute path only. The networkx oracle comparison
    // pre-builds the graph and times only the predicate, so parse must be
    // excluded to compare like with like.
    let g = parse_edge_list(GNM);
    c.bench_function("is_bipartite_gnm_2000_8000", |b| {
        b.iter(|| black_box(is_bipartite(black_box(&g))));
    });
    c.bench_function("is_forest_gnm_2000_8000", |b| {
        b.iter(|| black_box(is_forest(black_box(&g))));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
