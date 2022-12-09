use std::{
    collections::{HashSet},
    io::{self, BufRead},
    ops::Deref,
};

#[derive(Debug, Clone, Copy)]
enum Dir {
    R,
    U,
    L,
    D,
}

impl Dir {
    fn of(c: char) -> Dir {
        match c {
            'R' => Dir::R,
            'L' => Dir::L,
            'U' => Dir::U,
            'D' => Dir::D,
            _ => panic!("bad dir: {c}"),
        }
    }
    fn step(self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::R => (x + 1, y),
            Dir::U => (x, y + 1),
            Dir::L => (x - 1, y),
            Dir::D => (x, y - 1),
        }
    }
}

type Parsed = Vec<(Dir, usize)>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Parsed {
    lines
        .map(|line| {
            let line: &str = &*line;
            let (d, c) = line.split_once(' ').unwrap();
            (
                Dir::of(d.chars().next().unwrap()),
                c.parse::<usize>().unwrap(),
            )
        })
        .collect::<Parsed>()
}

fn part1(parsed: &Parsed) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);
    for &(d, c) in parsed {
        for _ in 0..c {
            head = d.step(head);
            tail = clamp(tail, head);
            visited.insert(tail);
        }
    }
    visited.len()
}

fn clamp(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() > 1 || dy.abs() > 1 {
        (tail.0 + dx.signum(), tail.1 + dy.signum())
    } else {
        tail
    }
}

fn part2(parsed: &Parsed) -> usize {
    let mut knots = [(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for &(d, c) in parsed {
        for _ in 0..c {
            knots[0] = d.step(knots[0]);
            let mut prev = knots[0];
            for k in &mut knots[1..] {
                *k = clamp(*k, prev);
                prev = *k;
            }
            visited.insert(prev);
        }
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    static SAMPLE_INPUT2: &str = "\
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";

    fn sample_lines() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    fn sample_lines2() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT2.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample_lines());
        assert_eq!(part1(&root), 13);
    }

    #[test]
    fn test_part2_1() {
        let root = parse(sample_lines());
        assert_eq!(part2(&root), 1);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample_lines2());
        assert_eq!(part2(&root), 36);
    }
}
