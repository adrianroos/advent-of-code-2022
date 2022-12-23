use std::{
    collections::HashSet,
    io::{self, BufRead},
};

type Parsed = HashSet<(i32, i32, i32)>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut result = Parsed::new();
    for line in lines {
        let line: &str = &*line;
        let mut coords = line.split(",").map(|x| x.parse::<i32>().expect(line));
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        result.insert((x, y, z));
    }
    result

    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

static NEIGHBORS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn part1(parsed: &Parsed) -> usize {
    let mut count = 0;
    for (x, y, z) in parsed.iter() {
        for (xx, yy, zz) in NEIGHBORS {
            if !parsed.contains(&(x + xx, y + yy, z + zz)) {
                count += 1;
            }
        }
    }
    count
}

fn fill(outside: &mut Parsed, parsed: &Parsed, (x, y, z): (i32, i32, i32)) {
    if !(x >= -1 && y >= -1 && z >= -1 && x < 21 && y < 21 && z < 21) {
        return;
    }
    if outside.contains(&(x, y, z)) || parsed.contains(&(x, y, z)) {
        return;
    }
    outside.insert((x, y, z));
    for (xx, yy, zz) in NEIGHBORS {
        fill(outside, parsed, (x + xx, y + yy, z + zz));
    }
}

fn part2(parsed: &Parsed) -> usize {
    if cfg!(debug_assertions) {
        panic!("fill will overflow the stack unless run with --release");
    }
    let mut count = 0;
    let mut outside = HashSet::<(i32, i32, i32)>::new();
    fill(&mut outside, parsed, (0, 0, 0));

    for (x, y, z) in parsed.iter() {
        for (xx, yy, zz) in NEIGHBORS {
            if outside.contains(&(x + xx, y + yy, z + zz)) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 64);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 58);
    }
}
