use dag::DirectedAcyclicGraph;

fn main() {
    let mut graph = DirectedAcyclicGraph::new();
    let a = graph.add_node();
    let b = graph.add_node();
    let c = graph.add_node();
    println!("{}", graph.add_edge(a, b));
    println!("{}", graph.add_edge(b, c));
    println!("{}", graph.add_edge(c, a));
    println!("{:?}", graph.topological_sort());
}
