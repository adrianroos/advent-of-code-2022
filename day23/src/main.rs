use std::{
    collections::{HashSet},
    io::{self, BufRead},
    ops::Deref,
};

type Elf = (i32, i32);
type Parsed = HashSet<Elf>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut result = Parsed::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert((x as i32, y as i32));
            }
        }
    }
    println!("{:?}", result.len());
    result
   
    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

fn next_pos(elfs: &Parsed, elf: Elf, off: usize) -> (i32, i32) {
    let (x, y) = elf;

    let scans = [
        [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)], // N
        [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)], // S
        [(x - 1, y), (x - 1, y - 1), (x - 1, y + 1)], // W
        [(x + 1, y), (x + 1, y - 1), (x + 1, y + 1)], // E
    ];
    
    
    if scans.iter().flatten().all(|elf| !elfs.contains(elf)) {
        return elf
    }

    for s in scans.iter().cycle().skip(off).take(scans.len()) {
        if s.iter().all(|x| !elfs.contains(x)) {
            return s[0];
        }
    }
    elf
}

fn part1(parsed: &Parsed) -> usize {
    let mut elfs = parsed.clone();
    simulate(&mut elfs, 0..10);

    let min_x = elfs.iter().map(|&(x, y)| x).min().unwrap();
    let max_x = elfs.iter().map(|&(x, y)| x).max().unwrap();
    let min_y = elfs.iter().map(|&(x, y)| y).min().unwrap();
    let max_y = elfs.iter().map(|&(x, y)| y).max().unwrap();

    let area = (max_x - min_x + 1) * (max_y - min_y + 1);

    area as usize - elfs.len()
}

fn simulate(elfs: &mut HashSet<(i32, i32)>, rounds: impl Iterator<Item = usize>) -> usize {
    let mut active = elfs.clone();
    for off in rounds {
        let mut next = Parsed::new();
        let mut dups = Parsed::new();
        let mut moved = false;

        for &elf in &*elfs {
            let n = next_pos(&*elfs, elf, off);
            if next.contains(&n) {
                dups.insert(n);
            } else {
                next.insert(n);
            }
        }

        next.clear();
    
        for &elf in &*elfs {
            let n = next_pos(&*elfs, elf, off);
            if !dups.contains(&n) && n != elf {
                next.insert(n);
                moved = true
            } else {
                next.insert(elf);
            }
        }

        if !moved {
            return off;
        }

        *elfs = next;
    }
    0
}

fn part2(parsed: &Parsed) -> usize {
    let mut elfs = parsed.clone();
    simulate(&mut elfs, 0..) + 1
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    ....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 110);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 20);
    }
}
