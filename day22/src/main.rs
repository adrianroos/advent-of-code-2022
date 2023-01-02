use core::panic;
use std::{
    fmt::{Debug, Display},
    io::{self, BufRead},
    iter::Peekable,
    ops::{Index, IndexMut},
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

    fn adv(&self, pos: (usize, usize), dir: (i64, i64)) -> (usize, usize) {
        let mut next = (pos.0 as i64 + dir.0, pos.1 as i64 + dir.1);
        next.0 = next.0.rem_euclid(self.dim.0 as i64);
        next.1 = next.1.rem_euclid(self.dim.1 as i64);
        (next.0 as usize, next.1 as usize)
    }

    fn width(&self) -> usize {
        self.dim.0
    }
    fn height(&self) -> usize {
        self.dim.1
    }
}

impl<T: Clone> Grid<T> {
    fn parse(lines: impl Iterator<Item = impl AsRef<str>>, map: impl Fn(char) -> T) -> Grid<T> {
        let lines = lines.collect::<Vec<_>>();
        let mut res = Grid::new((0, 0));
        let mut height = 0;
        res.dim.0 = lines
            .iter()
            .map(|x| x.as_ref().len())
            .max()
            .expect("Empty grid");
        assert_ne!(0, res.dim.0);
        for l in lines {
            let mut width = 0;
            for c in l.as_ref().chars() {
                res.vec.push(map(c));
                width += 1;
            }
            for _ in width..res.dim.0 {
                res.vec.push(map(' '));
            }
            height += 1;
        }
        res.dim.1 = height;
        res
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

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            dim: self.dim.clone(),
            vec: self.vec.clone(),
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.dim.0, "{x},{y} < {:?}", self.dim);
        assert!(y < self.dim.1, "{x},{y} < {:?}", self.dim);
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grid(w={}, h={}", self.width(), self.height())?;
        for y in 0..self.height() {
            writeln!(f)?;
            for x in 0..self.width() {
                self[(x, y)].fmt(f)?;
            }
        }
        Ok(())
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grid(w={}, h={}", self.width(), self.height())?;
        for y in 0..self.height() {
            writeln!(f)?;
            for x in 0..self.width() {
                self[(x, y)].fmt(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Field {
    Void,
    Wall,
    Free,
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, " "),
            Self::Wall => write!(f, "#"),
            Self::Free => write!(f, "."),
        }
    }
}

impl Field {
    fn parse(c: char) -> Field {
        match c {
            '.' => Field::Free,
            ' ' => Field::Void,
            '#' => Field::Wall,
            c => panic!("Unexpected {c}"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instr {
    Go(u32),
    L,
    R,
}

impl Instr {
    fn parse(it: &mut Peekable<impl Iterator<Item = char>>) -> Option<Instr> {
        match it.next() {
            Some('L') => Some(Instr::L),
            Some('R') => Some(Instr::R),
            Some(c) if c.is_ascii_digit() => {
                let mut steps = c.to_digit(10).unwrap();
                while let Some(&n @ '0'..='9') = it.peek() {
                    steps *= 10;
                    steps += n.to_digit(10).unwrap();
                    it.next();
                }
                Some(Instr::Go(steps))
            }
            None => None,
            Some(c) => panic!("Unexpected: {c}"),
        }
    }
    fn parse_line(l: &str) -> Vec<Instr> {
        let mut res = Vec::new();
        let mut l = l.chars().peekable();
        while let Some(i) = Self::parse(&mut l) {
            res.push(i);
        }
        res
    }
}

type Parsed = (Grid<Field>, Vec<Instr>);

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let lines = lines.collect::<Vec<_>>();
    let (grid, instr) = lines.split_at(lines.iter().position(|x| *x == "").unwrap());
    let grid = Grid::parse(grid.into_iter(), Field::parse);
    let instr = Instr::parse_line(&instr[1]);
    (grid, instr)
}

static DIRS: &[(i64, i64)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    R = 0,
    D = 1,
    L = 2,
    U = 3,
}

impl Dir {
    fn idx(self: Self) -> usize {
        match self {
            Dir::R => 0,
            Dir::D => 1,
            Dir::L => 2,
            Dir::U => 3,
        }
    }

    fn int(self: Self) -> i32 {
        self.idx() as i32
    }
}

impl From<i32> for Dir {
    fn from(i: i32) -> Self {
        Self::from(i.rem_euclid(4) as usize)
    }
}

impl From<usize> for Dir {
    fn from(v: usize) -> Self {
        match v {
            0 => Dir::R,
            1 => Dir::D,
            2 => Dir::L,
            3 => Dir::U,
            c => panic!("{c}"),
        }
    }
}

fn part1(parsed: &Parsed) -> usize {
    let (grid, instr) = parsed;
    let mut pos = grid.position(Field::Free).unwrap();
    let mut dir: i32 = 0;

    for &i in instr {
        match i {
            Instr::Go(c) => {
                'outer: for _ in 0..c {
                    let mut next = pos;
                    loop {
                        next = grid.adv(next, DIRS[dir as usize]);
                        match &grid[next] {
                            Field::Void => (),
                            Field::Wall => break 'outer,
                            Field::Free => {
                                pos = next;
                                break;
                            }
                        }
                    }
                }
            }
            Instr::L => dir = (dir - 1).rem_euclid(4),
            Instr::R => dir = (dir + 1).rem_euclid(4),
        }
    }

    (pos.0 + 1) * 4 + 1000 * (pos.1 + 1) + dir as usize
}

#[derive(Debug, Default)]
struct Face {
    /// (x, y) index of the face (i.e. top left corner divided by the face size).
    idx: (usize, usize),
    /// Neighbor faces of this face: (index, back_dir), by Dir::idx.
    /// back_dir: The dir to face to get back to self.
    neigh: [Option<((usize, usize), Dir)>; 4],
}

impl Face {
    /// Returns the coordinates and direction for entering a face on the cubenet with a given face [size],
    /// at the edge to the [from_dir] side of the face, with [old_pos] being the offset from the
    /// left side of the edge (in direction of travel).
    fn enter(&self, old_pos: usize, from_dir: Dir, size: usize) -> ((usize, usize), Dir) {
        // Direction we're facing now, i.e. into the face from the edge
        let new_dir = Dir::from(from_dir.int() + 2);
        // Top-left coordinate of the face
        let (xoff, yoff) = (self.idx.0 * size, self.idx.1 * size);
        /*

            old_pos origin
                 |
                 \-> x-------|
                     |   ^   |
                R -> |   U   | < L
                     |       |
                     --------x
                         v
                         D
        */

        (
            match new_dir {
                Dir::R => (xoff, yoff + old_pos),
                Dir::D => (xoff + size - 1 - old_pos, yoff),
                Dir::L => (xoff + size - 1, yoff + size - 1 - old_pos),
                Dir::U => (xoff + old_pos, yoff + size - 1),
            },
            new_dir,
        )
    }

    fn try_fill_all(f: &Face, faces: &mut [[Option<Face>; 4]; 4]) {
        Self::try_fill(f, faces, Dir::U, Dir::L);
        Self::try_fill(f, faces, Dir::U, Dir::R);
        Self::try_fill(f, faces, Dir::D, Dir::L);
        Self::try_fill(f, faces, Dir::D, Dir::R);
    }

    /// If [f] has neighbors both in directions [one_dir] and [two_dir], fills in their neighbor arrays
    fn try_fill(f: &Face, faces: &mut [[Option<Face>; 4]; 4], one_dir: Dir, two_dir: Dir) {
        if let (Some(one), Some(two)) = (f.neigh[one_dir.idx()], f.neigh[two_dir.idx()]) {
            // The direction to two from the view point of one, i.e. two_dir, rotatated by the dfiference between f's back_direction in one, and flipped one_dir.
            let one_rot = Dir::from(two_dir.int() + (one.1.int() - one_dir.int() - 2));
            let two_rot = Dir::from(one_dir.int() + (two.1.int() - two_dir.int() - 2));
            (&mut faces[one.0 .0][one.0 .1]).as_mut().unwrap().neigh[one_rot.idx()] =
                Some((two.0, two_rot));
            (&mut faces[two.0 .0][two.0 .1]).as_mut().unwrap().neigh[two_rot.idx()] =
                Some((one.0, one_rot));
        }
    }
}

fn part2(parsed: &Parsed) -> usize {
    let (grid, instr) = parsed;
    let size = if grid.width() <= 4 * 4 { 4 } else { 50 };

    let mut faces: [[Option<Face>; 4]; 4] = [[0; 4]; 4].map(|x| x.map(|_y| None));

    for y in 0..(grid.height() / size) {
        for x in 0..(grid.width() / size) {
            if grid[(x * size, y * size)] != Field::Void {
                let mut f = Face {
                    idx: (x, y),
                    neigh: [None; 4],
                };
                if x > 0 {
                    if let Some(n) = &mut faces[x - 1][y] {
                        n.neigh[Dir::R.idx()] = Some(((x, y), Dir::L));
                        f.neigh[Dir::L.idx()] = Some(((x - 1, y), Dir::R));
                    }
                }
                if y > 0 {
                    if let Some(n) = &mut faces[x][y - 1] {
                        n.neigh[Dir::D.idx()] = Some(((x, y), Dir::U));
                        f.neigh[Dir::U.idx()] = Some(((x, y - 1), Dir::D));
                    }
                }
                faces[x][y] = Some(f);
            }
        }
    }

    for _ in 0..6 {
        for x in 0..(grid.width() / size) {
            for y in 0..(grid.height() / size) {
                if let Some(f) = faces[x][y].take() {
                    Face::try_fill_all(&f, &mut faces);
                    faces[x][y] = Some(f);
                }
            }
        }
    }

    let mut pos = grid.position(Field::Free).unwrap();
    let mut dir = Dir::R;
    for &i in instr {
        match i {
            Instr::Go(steps) => {
                for _ in 0..steps {
                    let (x, y) = pos;
                    let next = match dir {
                        Dir::R => {
                            if x % size < size - 1 {
                                Ok(((x + 1, y), dir))
                            } else {
                                Err(y % size)
                            }
                        }
                        Dir::D => {
                            if y % size < size - 1 {
                                Ok(((x, y + 1), dir))
                            } else {
                                Err(size - 1 - x % size)
                            }
                        }
                        Dir::L => {
                            if x % size > 0 {
                                Ok(((x - 1, y), dir))
                            } else {
                                Err(size - 1 - y % size)
                            }
                        }
                        Dir::U => {
                            if y % size > 0 {
                                Ok(((x, y - 1), dir))
                            } else {
                                Err(x % size)
                            }
                        }
                    };

                    let next = match next {
                        Ok(next) => next,
                        Err(old_pos) => {
                            let f = faces[x as usize / size][y as usize / size]
                                .as_ref()
                                .unwrap();
                            let n = f.neigh[dir.idx()].unwrap();
                            let nf: &Face = &faces[n.0 .0][n.0 .1].as_ref().unwrap();
                            let mapped = nf.enter(old_pos, n.1, size);
                            // println!("Mapping from {pos:?} {dir:?} to {mapped:?} on {n:?}");
                            mapped
                        }
                    };

                    match grid[next.0] {
                        Field::Void => panic!("Walked into void from {pos:?} {dir:?} to {next:?}"),
                        Field::Wall => break,
                        Field::Free => (pos, dir) = next,
                    }
                }
            }
            Instr::L => dir = Dir::from(dir.int() - 1),
            Instr::R => dir = Dir::from(dir.int() + 1),
        }
    }

    (pos.0 + 1) * 4 + 1000 * (pos.1 + 1) + dir.idx()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 6032);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 5031);
    }
}
