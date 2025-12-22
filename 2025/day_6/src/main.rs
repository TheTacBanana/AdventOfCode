use grid::Grid;

const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn main() {
    let grid = Grid::from(
        INPUT
            .trim()
            .split_terminator("\n")
            .map(|row| row.split_ascii_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let mut p1_total = 0;
    for col in grid.iter_cols() {
        let col = col.cloned().collect::<Vec<_>>();
        let op = if col.last().cloned().unwrap() == "*" {
            Op::Mul
        } else {
            Op::Add
        };

        let val = col.iter().fold(op as u64, |l, r| {
            let Ok(r): Result<u64, _> = r.parse() else {
                return l;
            };
            if op == Op::Mul {
                l * r
            } else {
                l + r
            }
        });

        p1_total += val;
    }
    println!("Part 1: {p1_total}");

    let mut grid = Grid::from(
        INPUT
            .split_terminator('\n')
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let row = grid.remove_row(grid.rows() - 1).unwrap();
    grid.insert_row(0, row);
    grid.push_col((0..grid.rows()).map(|_| ' ').collect());

    let mut start_indexes = Vec::new();
    for (i, c) in grid.iter_row(0).enumerate() {
        if *c != ' ' {
            start_indexes.push(i);
        }
    }

    let mut p2_total = 0;
    let mut cur_op = None;
    let mut nums = Vec::new();
    for (i, col) in grid.iter_cols().enumerate() {
        let mut col = col.cloned().collect::<Vec<_>>();
        if Some(i) == start_indexes.first().copied() {
            cur_op = Some(if col[0] == '*' { Op::Mul } else { Op::Add });
            start_indexes.remove(0);
        }
        col.remove(0);

        let num = col.iter().fold(String::new(), |mut l, r| {
            l.push(*r);
            l
        });
        let num = num.trim();

        if num.is_empty() {
            let val = nums.drain(..).fold(cur_op.unwrap() as u64, |l, r| {
                if cur_op.unwrap() == Op::Mul {
                    l * r
                } else {
                    l + r
                }
            });
            p2_total += val;
        } else {
            nums.push(num.parse::<u64>().unwrap());
        }
    }

    println!("Part 2: {p2_total}");
}
