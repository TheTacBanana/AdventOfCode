use cgmath::{MetricSpace, Point3};
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::DfsPostOrder,
};

const INPUT: &str = include_str!("input");

fn main() {
    let points = INPUT
        .split_whitespace()
        .map(|l| {
            let iter = l
                .split_terminator(",")
                .map(|n| n.parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            Point3::new(iter[0], iter[1], iter[2])
        })
        .collect::<Vec<_>>();

    let mut graph = UnGraph::<Point3<f64>, (), _>::new_undirected();
    for p in points {
        graph.add_node(p);
    }

    fn make_connection(
        graph: &mut petgraph::Graph<Point3<f64>, (), petgraph::Undirected>,
    ) -> (NodeIndex, NodeIndex) {
        let mut min = f64::MAX;
        let mut points = None;
        for n1 in graph.node_indices() {
            for n2 in graph.node_indices() {
                if graph.contains_edge(n1, n2) || n1 == n2 {
                    continue;
                }

                let (l, r) = (
                    graph.node_weight(n1).unwrap(),
                    graph.node_weight(n2).unwrap(),
                );

                let dist = l.distance(*r);
                if dist < min {
                    min = dist;
                    points = Some((n1, n2));
                }
            }
        }

        let (n1, n2) = points.unwrap();
        graph.add_edge(n1, n2, ());
        (n1, n2)
    }

    for _ in 0..1000 {
        _ = make_connection(&mut graph)
    }

    let mut removal_graph = graph.clone();

    let mut subgraphs = Vec::new();
    while let Some(start) = removal_graph.node_indices().next() {
        let mut dfs = DfsPostOrder::new(&removal_graph, start);

        let mut component = Vec::new();
        while let Some(n) = dfs.next(&removal_graph) {
            component.push(n);
        }
        for n in component.iter() {
            removal_graph.remove_node(*n);
        }
        subgraphs.push(component.len() as u64);
    }
    subgraphs.sort();
    subgraphs.reverse();

    let p1_total: u64 = subgraphs[0] * subgraphs[1] * subgraphs[2];
    println!("Part 1: {p1_total}");

    let (n1, n2) = loop {
        let added = make_connection(&mut graph);

        let first = graph.node_indices().next().unwrap();
        let mut dfs = DfsPostOrder::new(&graph, first);
        let mut total = 0;
        while let Some(_) = dfs.next(&graph) {
            total += 1;
        }

        if graph.node_count() == total {
            break added;
        }
    };

    let p2_val = graph.node_weight(n1).unwrap().x * graph.node_weight(n2).unwrap().x;
    println!("Part 1: {p2_val}");
}
