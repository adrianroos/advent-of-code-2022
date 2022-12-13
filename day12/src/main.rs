use std::{
    collections::VecDeque,
    io::{self, BufRead},
    ops::{Deref, Index, IndexMut},
};

struct Grid<T> {
    dim: (usize, usize),
    vec: Vec<T>,
}

impl<T> Grid<T> {
    fn new(dim: (usize, usize)) -> Grid<T> {
        Grid {
            dim,
            vec: Vec::new(),
        }
    }

    fn clone_with<U: Copy>(&self, u: U) -> Grid<U> {
        Grid {
            dim: self.dim,
            vec: vec![u; self.vec.len()],
        }
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if x > 0 {
            v.push((x - 1, y));
        }
        if y > 0 {
            v.push((x, y - 1));
        }
        if x < self.dim.0 - 1 {
            v.push((x + 1, y));
        }
        if y < self.dim.1 - 1 {
            v.push((x, y + 1));
        }
        v
    }
}

impl<T: PartialEq> Grid<T> {
    fn position(&self, needle: T) -> Option<(usize, usize)> {
        let p = self.vec.iter().position(|x| *x == needle);
        p.map(|p| (p % self.dim.0, p / self.dim.0))
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.dim.0);
        assert!(y < self.dim.1);
        let width = self.dim.0;
        &self.vec[y * width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.dim.0);
        assert!(y < self.dim.1);
        let width = self.dim.0;
        &mut self.vec[y * width + x]
    }
}

type Parsed = Grid<char>;

fn height(c: char) -> usize {
    (match c {
        'S' => 'a',
        'E' => 'z',
        c => c,
    }) as usize
        - 'a' as usize
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Parsed {
    let mut result = Parsed::new((0, 0));
    for line in lines {
        let line: &str = &*line.trim();
        result.dim.0 = line.len();
        for c in line.chars() {
            result.vec.push(c)
        }
    }
    result.dim.1 = result.vec.len() / result.dim.0;
    result
}

fn part1(parsed: &Parsed) -> usize {
    let mut q = VecDeque::new();
    let mut d = parsed.clone_with(usize::max_value());
    let start = parsed.position('S').unwrap();
    q.push_back(start);
    d[start] = 0;
    while let Some(p) = q.pop_front() {
        let dn = d[p] + 1;
        for n in parsed.neighbors(p) {
            if height(parsed[n]) <= height(parsed[p]) + 1 {
                if dn < d[n] {
                    d[n] = dn;
                    q.push_back(n);
                }
            }
        }
    }
    d[parsed.position('E').unwrap()]
}

fn part2(parsed: &Parsed) -> usize {
    let mut q = VecDeque::new();
    let mut d = parsed.clone_with(usize::max_value());
    let end = parsed.position('E').unwrap();
    q.push_back(end);
    d[end] = 0;
    while let Some(p) = q.pop_front() {
        if height(parsed[p]) == height('a') {
            return d[p];
        }
        let dn = d[p] + 1;
        for n in parsed.neighbors(p) {
            if height(parsed[n]) + 1 >= height(parsed[p]) {
                if dn < d[n] {
                    d[n] = dn;
                    q.push_back(n);
                }
            }
        }
    }
    d[parsed.position('E').unwrap()]
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    fn sample() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 31);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 36);
    }
}
