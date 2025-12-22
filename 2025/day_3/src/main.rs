use cached::proc_macro::cached;

const INPUT: &str = include_str!("input");

fn main() {
    let iter = INPUT.trim().split_ascii_whitespace().collect::<Vec<_>>();

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for line in iter.clone() {
        let nums = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();

        p1_sum += recurse(nums.clone(), 2);
        p2_sum += recurse(nums, 12);
    }
    println!("Part 1: {p1_sum}");
    println!("Part 2: {p2_sum}");
}

#[cached]
fn recurse(line: Vec<u64>, n: usize) -> u64 {
    if n == 0 || line.len() < n {
        return 0;
    }

    let mut max = 0;
    for (i, l) in line[..=(line.len() - n)].iter().enumerate() {
        let mut vec = Vec::new();
        line[(i + 1)..].clone_into(&mut vec);
        max = u64::max(max, l * 10u64.pow((n - 1) as u32) + recurse(vec, n - 1))
    }

    return max;
}
