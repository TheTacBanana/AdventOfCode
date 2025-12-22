use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    let iter = INPUT.trim_end().split_terminator(",").collect::<Vec<_>>();

    let mut invalid_total_p1 = 0u64;
    let mut invalid_total_p2 = 0u64;

    for i in iter {
        let (start, end) = i.split_at(i.find("-").unwrap());
        let (start, end): (u64, u64) = (start.parse().unwrap(), end[1..].parse().unwrap());

        for i in start..=end {
            if !check_valid_p1(i) {
                invalid_total_p1 += i;
            }

            if !check_valid_p2(i) {
                invalid_total_p2 += i;
            }
        }
    }

    println!("Part 1: {invalid_total_p1}");
    println!("Part 2: {invalid_total_p2}");
}

fn check_valid_p1(i: u64) -> bool {
    let s = i.to_string();

    let (l, r) = s.split_at(s.len() / 2);

    !(l == r)
}

fn check_valid_p2(num: u64) -> bool {
    let s = num.to_string();
    let vec = s.chars().collect::<Vec<_>>();

    for i in 1..=s.len() {
        if i == s.len() {
            break;
        }
        let set = vec.chunks(i).collect::<HashSet<_>>();

        if set.len() == 1 {
            return false;
        }
    }
    true
}
