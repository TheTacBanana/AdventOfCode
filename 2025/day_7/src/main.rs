use grid::Grid;

const INPUT: &str = include_str!("input");

fn main() {
    let mut grid = Grid::from(
        INPUT
            .split_terminator('\n')
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let mut timelines = (0..grid.cols()).map(|_| 0u64).collect::<Vec<_>>();
    let mut p1_count = 0;
    for row in 1..grid.rows() {
        for col in 0..grid.cols() {
            match grid.get(row, col).unwrap() {
                '^' => {
                    if *grid.get(row - 1, col).unwrap() == '|' {
                        *grid.get_mut(row, col - 1).unwrap() = '|';
                        *grid.get_mut(row, col + 1).unwrap() = '|';
                        p1_count += 1;

                        let n = timelines[col];
                        timelines[col] = 0;
                        timelines[col - 1] += n;
                        timelines[col + 1] += n;
                    }
                }
                _ => (),
            }
            match grid.get(row - 1, col).unwrap() {
                'S' => {
                    *grid.get_mut(row, col).unwrap() = '|';
                    timelines[col] = 1;
                }
                '|' => {
                    let cell = grid.get_mut(row, col).unwrap();
                    if *cell == '.' {
                        *cell = '|';
                    }
                }
                _ => (),
            }
        }
    }

    let p2_sum: u64 = timelines.iter().sum();

    println!("Part 1: {p1_count}");
    println!("Part 2: {p2_sum}");
}
