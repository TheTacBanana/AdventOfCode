use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input");

fn main() {
    let points = INPUT
        .split_whitespace()
        .map(|l| {
            let iter = l
                .split_terminator(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (iter[0], iter[1])
        })
        .collect::<Vec<_>>();

    let mut max_area = 0;
    for i in points.iter() {
        for j in points.iter() {
            let x_dif = (j.0 - i.0).abs() + 1;
            let y_dif = (j.1 - i.1).abs() + 1;

            let area = x_dif * y_dif;
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Part 1: {max_area}");

    let mut unique_x = points
        .iter()
        .map(|p| p.0)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_x.sort();
    let mut x_trans = Trans::default();

    for (i, &x) in unique_x.iter().enumerate() {
        x_trans.forw.insert(x, i as i64);
        x_trans.rev.insert(i as i64, x);
    }

    let mut unique_y = points
        .iter()
        .map(|p| p.1)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_y.sort();
    let mut y_trans = Trans::default();

    for (i, &y) in unique_y.iter().enumerate() {
        y_trans.forw.insert(y, i as i64);
        y_trans.rev.insert(i as i64, y);
    }

    let points = points
        .into_iter()
        .map(|(x, y)| (x_trans.forw(x), y_trans.forw(y)))
        .collect::<Vec<_>>();

    let polygon = {
        let mut map = HashSet::<(i64, i64)>::new();

        for x in 0..unique_x.len() as i64 {
            for y in 0..unique_y.len() as i64 {
                if point_in_polygon((x, y), &points) {
                    map.insert((x, y));
                }
            }
        }
        map
    };

    println!("{points:?}");

    let mut max_area = 0;
    let mut max_points = ((i64::MIN, i64::MIN), (i64::MIN, i64::MIN));
    for i in points.iter() {
        'outer: for j in points.iter() {
            if i == j {
                continue;
            }

            let p1 = (x_trans.rev(i.0), y_trans.rev(i.1));
            let p2 = (x_trans.rev(j.0), y_trans.rev(j.1));

            let x_dif = (p2.0 - p1.0).abs() + 1;
            let y_dif = (p2.1 - p1.1).abs() + 1;

            let area = x_dif * y_dif;
            if area < max_area {
                continue;
            }

            let min_x = i64::min(i.0, j.0);
            let min_y = i64::min(i.1, j.1);
            let x_dif = (j.0 - i.0).abs() + 1;
            let y_dif = (j.1 - i.1).abs() + 1;

            for x in min_x..(min_x + x_dif) {
                for y in min_y..(min_y + y_dif) {
                    if !polygon.contains(&(x, y)) {
                        continue 'outer;
                    }
                }
            }

            max_area = area;
            max_points = (p1, p2);
        }
    }

    let (p1, p2) = max_points;

    let x_dif = (p2.0 - p1.0).abs() + 1;
    let y_dif = (p2.1 - p1.1).abs() + 1;

    println!("Part 2: {:?}", x_dif * y_dif)
}

#[derive(Default)]
struct Trans {
    forw: HashMap<i64, i64>,
    rev: HashMap<i64, i64>,
}

impl Trans {
    fn forw(&self, i: i64) -> i64 {
        *self.forw.get(&i).unwrap()
    }

    fn rev(&self, i: i64) -> i64 {
        *self.rev.get(&i).unwrap()
    }
}

fn point_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
    if cross != 0 {
        return false;
    }

    px >= x1.min(x2) && px <= x1.max(x2) && py >= y1.min(y2) && py <= y1.max(y2)
}

fn point_in_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (px, py) = point;
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if point_on_segment(px, py, x1, y1, x2, y2) {
            return true;
        }

        let intersects = (y1 > py) != (y2 > py);
        if intersects {
            let x_intersect = (x2 - x1) * (py - y1) / (y2 - y1) + x1;

            if px < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
}
