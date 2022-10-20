// see https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/

use petgraph::graph::Graph;

fn main() {
    let mut graph = Graph::<(), ()>::new();
    graph.extend_with_edges(&[ (0, 1) ]);
    assert_eq!(graph.node_count(), 2);
}
