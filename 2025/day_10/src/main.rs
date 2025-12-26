use petgraph::{data::DataMap, prelude::DiGraphMap};
use std::{
    collections::{HashMap, VecDeque},
    iter::Sum,
};
use z3::{
    ast::{Ast, Int, IntoAst},
    Optimize,
};

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Input {
    pub target_state: TargetState,
    pub buttons_state: Vec<ButtonBits>,

    pub target_joltage: Vec<u16>,
    pub buttons_joltage: Vec<Vec<u16>>,
}

type TargetState = u16;
type ButtonBits = u16;

fn main() {
    let inputs = INPUT
        .trim()
        .split_terminator("\n")
        .map(|l| {
            let iter = l.split_whitespace().collect::<Vec<_>>();

            let target = &iter[0][1..=iter[0].len() - 2];
            let buttons = &iter[1..=iter.len() - 2];
            let joltage = &iter[iter.len() - 1][1..=iter[iter.len() - 1].len() - 2];

            let target_state = target
                .chars()
                .enumerate()
                .fold(0u16, |l, (i, r)| l | (((r == '#') as u16) << i));

            let buttons_state = buttons
                .iter()
                .map(|s| {
                    let s = &s[1..=s.len() - 2];

                    s.split_terminator(",")
                        .fold(0u16, |l, r| l | (1 << r.parse::<u16>().unwrap()))
                })
                .collect::<Vec<_>>();

            let target_joltage = joltage
                .split_terminator(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let buttons_joltage = buttons
                .iter()
                .map(|s| {
                    let s = &s[1..=s.len() - 2];

                    s.split_terminator(",")
                        .map(|s| s.parse().unwrap())
                        .collect()
                })
                .collect::<Vec<_>>();

            Input {
                target_state,
                buttons_state,
                target_joltage,
                buttons_joltage,
            }
        })
        .collect::<Vec<_>>();

    let mut p1_total = 0;

    for Input {
        target_state: target,
        buttons_state: buttons,
        ..
    } in &inputs
    {
        let mut graph = DiGraphMap::<TargetState, ButtonBits>::new();

        let start_node = graph.add_node(TargetState::default());
        let target_node = graph.add_node(*target);

        let mut queue = buttons
            .iter()
            .map(|&b| (start_node, b))
            .collect::<VecDeque<_>>();

        while let Some((node, button)) = queue.pop_front() {
            let mut weight = graph.node_weight(node).unwrap().clone();

            if graph.edges(node).any(|e| *e.2 == button) {
                continue;
            }

            weight ^= button;

            let new_node = graph.add_node(weight);
            graph.add_edge(node, new_node, button);

            if new_node == target_node {
                break;
            }

            queue.extend(buttons.iter().map(|&b| (new_node, b)));
        }

        use petgraph::algo::astar;

        let path = astar(
            &graph,
            start_node,
            |finish| finish == target_node,
            |_| 1,
            |_| 0,
        );

        p1_total += path.unwrap().0;
    }

    println!("Part 1: {p1_total}");

    let mut p2_total = 0;
    for Input {
        target_joltage: target,
        buttons_joltage: buttons,
        ..
    } in inputs
    {
        let opt = Optimize::new();

        let mut vars = HashMap::new();
        for (i, button) in buttons.iter().enumerate() {
            let var = Int::fresh_const(&format!("b{i}"));
            opt.assert(&var.ge(0));

            vars.insert(var, button.clone());
        }

        let mut outs = (0..target.len()).map(|_| Vec::new()).collect::<Vec<_>>();

        for (var, adds) in vars.clone() {
            for x in adds {
                outs[x as usize].push(var.clone());
            }
        }

        // println!("{outs:?}");

        for (l, r) in target.iter().zip(outs) {
            opt.assert(&Int::sum(r.iter()).eq(*l));
        }

        let sum = Int::sum(vars.keys());
        opt.minimize(&sum);

        opt.check(&[]);

        let model = opt.get_model().unwrap();

        let val = vars
            .keys()
            .map(|button| model.eval(button, true).unwrap().as_i64().unwrap())
            .sum::<i64>();

        p2_total += val;
    }

    println!("Part 2: {p2_total}");
}

// fn astar_search(
//     start: TargetJoltage,
//     target: TargetJoltage,
//     buttons: Vec<ButtonJoltage>,
// ) -> Option<i32> {
//     use pathfinding::prelude::astar;
//     let result = astar(
//         &start,
//         |p: &TargetJoltage| {
//             buttons
//                 .iter()
//                 .filter_map(|b| {
//                     let mut next = *p;
//                     for (i, (n, &delta)) in next.iter_mut().zip(b.iter()).enumerate() {
//                         *n += delta;
//                         if *n > target[i] {
//                             return None;
//                         }
//                     }
//                     Some((next, 1))
//                 })
//                 .collect::<Vec<_>>()
//         },
//         |p| {
//             let mut heuristic = 0;
//             for (l, r) in p.iter().zip(target.iter()) {
//                 if l > r {
//                     return i32::MAX;
//                 }
//                 heuristic += (r - l) as i32
//             }
//             heuristic
//         },
//         |p| *p == target,
//     );

//     result.map(|i| i.1)
// }
