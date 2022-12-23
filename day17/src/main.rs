use std::{
    fmt::{Debug, Write},
    io::{self, BufRead},
    iter::repeat,
    ops::{Index, IndexMut, Range},
};

#[derive(Clone, Copy, Debug)]
enum Dir {
    L,
    R,
}

impl Dir {
    fn parse(s: &str) -> Vec<Dir> {
        s.chars()
            .map(|c| match c {
                '<' => Dir::L,
                '>' => Dir::R,
                c => panic!("unexpected: {c}"),
            })
            .collect::<Vec<_>>()
    }
}

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

    fn width(&self) -> usize {
        self.dim.0
    }
    fn height(&self) -> usize {
        self.dim.1
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

    fn parse(input: &str, map: impl Fn(char) -> T) -> Grid<T> {
        let mut res = Grid::new((0, 0));
        let mut height = 0;
        for l in input.lines() {
            let l = l.trim();
            let mut width = 0;
            for c in l.chars() {
                res.vec.push(map(c));
                width += 1;
            }
            if res.dim.0 != 0 {
                assert_eq!(res.dim.0, width);
            } else {
                assert_ne!(0, width);
                res.dim.0 = width;
            }
            height += 1;
        }
        res.dim.1 = height;
        res
    }
}

impl<T: Clone> Grid<T> {
    fn grow_to_height(&mut self, height: usize, value: T) {
        if height >= self.dim.1 {
            self.vec
                .extend(repeat(value).take((height - self.dim.1) * self.dim.0));
            self.dim.1 = height;
        }
    }

    fn clone_y_range(&self, ys: Range<usize>) -> Grid<T> {
        let mut result = Grid::new(self.dim);
        result.dim.1 = ys.len();
        result
            .vec
            .extend_from_slice(&self.vec[ys.start * self.width()..ys.end * self.width()]);
        result
    }
}

impl<T: PartialEq> Grid<T> {
    fn position(&self, needle: T) -> Option<(usize, usize)> {
        let p = self.vec.iter().position(|x| *x == needle);
        p.map(|p| (p % self.dim.0, p / self.dim.0))
    }
}

impl<T: PartialEq<T>> PartialEq<Self> for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dim == other.dim && self.vec == other.vec
    }
}

impl<T: Eq> Eq for Grid<T> {}

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

impl Debug for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grid(w={}, h={}", self.width(), self.height())?;
        for y in (0..self.height()).rev() {
            writeln!(f)?;
            for x in 0..self.width() {
                f.write_char(if self[(x, y)] { '#' } else { '.' })?;
            }
        }
        Ok(())
    }
}

type Parsed = Vec<Dir>;

const fn map_char(c: char) -> bool {
    match c {
        '#' => true,
        _ => false,
    }
}

fn get_blocks() -> Vec<Grid<bool>> {
    vec![
        Grid::parse("####", self::map_char),
        Grid::parse(
            "\
            .#.
            ###
            .#.",
            self::map_char,
        ),
        Grid::parse(
            "\
            ###
            ..#
            ..#",
            self::map_char,
        ),
        Grid::parse(
            "\
            #
            #
            #
            #",
            self::map_char,
        ),
        Grid::parse(
            "\
            ##
            ##",
            self::map_char,
        ),
    ]
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut lines = lines;
    Dir::parse(&lines.next().unwrap())
}

fn part1(parsed: &Parsed) -> usize {
    let blocks = get_blocks();
    let mut grid = Grid::<bool>::new((7, 0));
    let mut lava = parsed.iter().cloned().cycle();
    for i in 0..2022 {
        let block = &blocks[i % blocks.len()];
        let (mut x, mut y) = (2, grid.height() + 3);
        loop {
            match lava.next().unwrap() {
                Dir::L if x > 0 && !collide(block, &grid, x - 1, y) => x -= 1,
                Dir::R if !collide(block, &grid, x + 1, y) => x += 1,
                _ => (),
            }
            if y == 0 || collide(block, &grid, x, y - 1) {
                settle(block, &mut grid, x, y);
                break;
            }
            y -= 1;
        }
        // println!("{:?}", grid);
    }
    grid.height()
}

fn collide(block: &Grid<bool>, grid: &Grid<bool>, x: usize, y: usize) -> bool {
    if x + block.width() > grid.width() {
        return true;
    }
    for yy in 0..block.height() {
        if y + yy >= grid.height() {
            continue;
        }
        for xx in 0..block.width() {
            if block[(xx, yy)] && grid[(x + xx, y + yy)] {
                return true;
            }
        }
    }
    false
}

fn settle(block: &Grid<bool>, grid: &mut Grid<bool>, x: usize, y: usize) -> bool {
    grid.grow_to_height(y + block.height(), false);
    for yy in 0..block.height() {
        for xx in 0..block.width() {
            if block[(xx, yy)] {
                grid[(x + xx, y + yy)] = true;
            }
        }
    }
    false
}

fn part2(parsed: &Parsed) -> usize {
    let target = 1000000000000;

    let blocks = get_blocks();
    let mut grid = Grid::<bool>::new((7, 0));
    let mut lava = parsed.iter().cloned().enumerate().cycle().peekable();
    let mut block = blocks.iter().enumerate().cycle().peekable();

    // state: (lava pos, block pos, last 27 lines)

    let init = 122;

    for _ in 0..init {
        simulate(&mut grid, &mut lava, &mut block);
        // println!("{:?}", grid);
    }

    let h = grid.height();
    let state = (
        lava.peek().unwrap().0,
        block.peek().unwrap().0,
        grid.clone_y_range(grid.height() - 35..grid.height()),
    );

    let mut cycle_len = 0;
    for c in 1.. {
        simulate(&mut grid, &mut lava, &mut block);
        if state
            == (
                lava.peek().unwrap().0,
                block.peek().unwrap().0,
                grid.clone_y_range(grid.height() - 35..grid.height()),
            )
        {
            cycle_len = c;
            break;
        }
    }

    let delta = grid.height() - h;
    let cycles = (target - init) / cycle_len;
    let from_cycles = (cycles - 1) * delta;
    let remaining = target - init - cycle_len * cycles;

    println!("{delta}, {cycle_len}, {cycles}, {from_cycles}, {remaining}");

    for _ in 0..remaining {
        simulate(&mut grid, &mut lava, &mut block);
    }
    grid.height() + from_cycles
}

fn simulate<'a>(
    grid: &mut Grid<bool>,
    lava: &mut impl Iterator<Item = (usize, Dir)>,
    block: &mut impl Iterator<Item = (usize, &'a Grid<bool>)>,
) {
    let block = block.next().unwrap().1;
    let (mut x, mut y) = (2, grid.height() + 3);
    loop {
        match lava.next().unwrap().1 {
            Dir::L if x > 0 && !collide(block, &*grid, x - 1, y) => x -= 1,
            Dir::R if !collide(block, &*grid, x + 1, y) => x += 1,
            _ => (),
        }
        if y == 0 || collide(block, &*grid, x, y - 1) {
            settle(block, grid, x, y);
            break;
        }
        y -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 3068);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 1514285714288);
    }
}
