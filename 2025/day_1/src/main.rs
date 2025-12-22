const INPUT: &str = include_str!("input");

fn main() {
    let iter = INPUT.split_whitespace();

    let mut rot: i32 = 50;
    let mut count_p1 = 0;
    let mut count_p2 = 0;

    for i in iter {
        let n: i32 = i[1..].parse().unwrap();

        let mut movement: i32 = match i.chars().nth(0).unwrap() {
            'L' => -n,
            'R' => n,
            _ => panic!(),
        };

        while movement != 0 {
            rot += movement.signum();
            movement -= movement.signum();

            if rot == -1 {
                rot = 99;
            } else if rot == 100 {
                rot = 0;
            }

            if rot == 0 {
                count_p2 += 1;
            }
        }

        if rot == 0 {
            count_p1 += 1;
        }
    }

    println!("Part 1: {:?}", count_p1);
    println!("Part 2: {:?}", count_p2);
}
