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

    println!("Part 1: {max_area}")
}
