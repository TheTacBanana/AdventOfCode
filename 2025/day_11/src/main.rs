use memoize::memoize;
use petgraph::{algo::all_simple_paths, prelude::DiGraphMap};
use std::{collections::HashMap, hash::RandomState};

const INPUT: &str = include_str!("input");

fn main() {
    let lines = INPUT
        .trim()
        .split_terminator("\n")
        .map(|s| {
            let (l, r) = s.split_once(":").unwrap();

            (l, r.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let mut graph = DiGraphMap::<&str, ()>::new();

    for (from, conn) in lines {
        let from = graph.add_node(from);

        for to in conn {
            graph.add_node(to);
            graph.add_edge(from, to, ());
        }
    }

    let start = graph.add_node("you");
    let end = graph.add_node("out");

    let p1_result = all_simple_paths::<Vec<_>, _, RandomState>(&graph, start, end, 0, None)
        .collect::<Vec<_>>()
        .len();

    println!("Part 1: {}", p1_result);

    let map = INPUT
        .trim()
        .split_terminator("\n")
        .map(|s| {
            let (l, r) = s.split_once(":").unwrap();

            (
                l.to_string(),
                r.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    #[memoize(Ignore: map)]
    fn dfs(
        map: &HashMap<String, Vec<String>>,
        node: String,
        mut dac: bool,
        mut fft: bool,
    ) -> usize {
        if node == "out" && dac && fft {
            return 1;
        }

        if node == "dac" {
            dac = true;
        }
        if node == "fft" {
            fft = true;
        }

        let mut sum = 0;
        if let Some(nodes) = map.get(&node) {
            for next in nodes {
                sum += dfs(map, next.clone(), dac, fft);
            }
        }

        return sum;
    }

    let p2_result = dfs(&map, "svr".to_string(), false, false);
    println!("Part 2: {}", p2_result);
}
