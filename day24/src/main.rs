use core::panic;
use std::{
    collections::{HashSet, HashMap, VecDeque},
    io::{self, BufRead},
    ops::Deref,
};

#[derive(Copy, Clone, PartialEq, Eq)]
struct Blizz {
    start: (i32, i32),
    dir: (i32, i32),
}


impl Blizz {
    fn parse(pos: (i32, i32), c: char) -> Option<Blizz> {
        let dir = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            '#' | '.' => return None,
            c => panic!("{c:?}"),
        };
        Some(Blizz { start: (pos.0, pos.1), dir })
    }

    fn at_t(self, parsed: &Parsed, t: i32) -> Blizz {
        let (x, y) = self.start;
        let (dx, dy) = self.dir;
        let (x, y) = (x + dx * t, y + dy * t);

        let x = x.rem_euclid(parsed.1.0 as i32);
        let y = y.rem_euclid(parsed.1.1 as i32);

        Blizz { start: (x, y), dir: self.dir }
    }
}

type Parsed = (Vec<Blizz>, (i32, i32));

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut res = Parsed::default();
    let mut w = 0;
    let mut h = 0;
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(b) = Blizz::parse((x as i32 - 1, y as i32 - 1), c) {
                res.0.push(b)
            }
        }
        w = line.len();
        h += 1;
    }
    res.1 = ((w - 2) as i32, h - 2);
    println!("{:?}", res.1);
    res

    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

type State = ((i32, i32), i32);

fn part1(parsed: &Parsed) -> usize {
    let (w, h) = parsed.1;
    let start = ((0, -1), 0);
    let end = (w - 1, h);
    solve(&parsed, start, end).1 as usize
}

fn solve(parsed: &Parsed, start: State, end: (i32, i32)) -> State {
    let (w, h) = parsed.1;

    let mut q = VecDeque::new();
    let mut v = HashSet::new();
    q.push_back(start);
    v.insert(start);

    while let Some(s) = q.pop_front() {
        if s.0 == end {
            return s
        }

        let (x, y) = s.0;
        let nt = s.1 + 1;
        let next_states = [
            (((x+1, y), nt), 'R'),
            (((x-1, y), nt), 'L'),
            (((x, y+1), nt), 'D'),
            (((x, y-1), nt), 'U'),
            (((x, y), nt), 'W'),
        ];

        for (n, c) in next_states {
            if parsed.0.iter().any(|b| b.at_t(parsed, n.1).start == n.0) {
                continue;
            }
            if v.contains(&n) {
                continue;
            }
            if n.0 == start.0 || n.0 == end || 0 <= n.0.0 && n.0.0 < w && 0 <= n.0.1 && n.0.1 < h { 
                q.push_back(n);
                v.insert(n);
            }
        }
    }

    panic!("unreachable");
}

fn part2(parsed: &Parsed) -> usize {
    let (w, h) = parsed.1;
    let start = ((0, -1), 0);
    let end = (w - 1, h);
    let there = solve(&parsed, start, end);
    let back = solve(&parsed, there, start.0);
    solve(&parsed, back, end).1 as usize
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 18);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 54);
    }
}
