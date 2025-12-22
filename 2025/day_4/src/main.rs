use grid::Grid;

const INPUT: &str = include_str!("input");

fn main() {
    let mut old_grid = Grid::from(
        INPUT
            .trim()
            .split_ascii_whitespace()
            .map(|row| row.chars().map(|c| c == '@').collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let mut cur_grid = old_grid.clone();

    let mut p1_total = 0;
    let mut p2_total = 0;
    let mut n = 1;
    loop {
        for ((row, col), val) in old_grid.indexed_iter() {
            if !val {
                continue;
            }

            let mut sum = 0i32;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    sum += old_grid
                        .get(row as i32 + i, col as i32 + j)
                        .cloned()
                        .unwrap_or_default() as i32;
                }
            }

            if sum < 4 {
                p2_total += 1;
                *cur_grid.get_mut(row, col).unwrap() = false;
            }
        }

        if n == 1 {
            p1_total = p2_total;
        }
        n += 1;

        if old_grid == cur_grid {
            break;
        }
        old_grid = cur_grid.clone();
    }
    println!("Part 1: {p1_total}");
    println!("Part 2: {p2_total}");
}
