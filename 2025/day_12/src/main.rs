const INPUT: &str = include_str!("input");

fn main() {
    let lines = INPUT.trim().split_terminator("\n\n").collect::<Vec<_>>();
    let shapes = &lines[0..=5];
    let regions = lines[6];

    let shapes = {
        let mut temp = Vec::new();
        for shape in shapes {
            let mut parts = Vec::new();
            let mut iter = shape.split_terminator("\n").into_iter();
            iter.next().unwrap();

            for (i, line) in iter.enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        parts.push((j, i));
                    }
                }
            }

            temp.push(Shape { parts });
        }
        temp
    };

    let regions = {
        let mut temp = Vec::new();

        for line in regions.split_terminator("\n") {
            let (area, numbers) = line.split_once(": ").unwrap();

            let (width, length) = area.split_once("x").unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();

            temp.push(Region {
                size: (width.parse().unwrap(), length.parse().unwrap()),
                shapes: numbers.try_into().unwrap(),
            });
        }
        temp
    };

    let regions = regions
        .into_iter()
        .filter(|r| {
            r.shapes
                .iter()
                .enumerate()
                .map(|(i, n)| shapes[i].area() * n)
                .sum::<usize>()
                < r.area()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", regions.len());
}

#[derive(Debug)]
struct Shape {
    parts: Vec<(usize, usize)>,
}

impl Shape {
    fn area(&self) -> usize {
        self.parts.len()
    }
}

#[derive(Debug)]
struct Region {
    size: (usize, usize),
    shapes: [usize; 6],
}
impl Region {
    fn area(&self) -> usize {
        self.size.0 * self.size.1
    }
}
