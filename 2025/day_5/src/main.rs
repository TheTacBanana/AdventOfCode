const INPUT: &str = include_str!("input");

fn main() {
    let (ranges, ids) = INPUT.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .split_ascii_whitespace()
        .map(|s| {
            let (l, r) = s.split_once("-").unwrap();
            (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let ids: Vec<u64> = ids
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let mut p1_total = 0;
    for id in ids.iter() {
        for (l, r) in ranges.iter() {
            if l <= id && r >= id {
                p1_total += 1;
                break;
            }
        }
    }
    println!("Part 1: {p1_total}");

    ranges.sort();
    let mut complete = Vec::new();
    let mut cur_range = ranges[0];
    for (l2, r2) in ranges.drain(1..) {
        let (l1, r1) = cur_range;

        if l2 > r1 {
            complete.push(cur_range);
            cur_range = (l2, r2)
        } else {
            cur_range = (l1, u64::max(r1, r2))
        }
    }
    complete.push(cur_range);

    let mut p2_total = 0;
    for (l, r) in complete.iter() {
        p2_total += r - l + 1;
    }
    println!("Part 2: {p2_total}");
}
