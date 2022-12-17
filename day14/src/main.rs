use std::{
    io::{self, BufRead},
    iter::once,
};

type Parsed = Vec<Vec<(usize, usize)>>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    lines
        .map(|line| {
            line.split(" -> ")
                .map(|x| x.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Parsed>()
}

fn creep(creeper: &mut usize, dest: usize) {
    if *creeper < dest {
        *creeper += 1;
    } else if *creeper > dest {
        *creeper -= 1;
    }
}

fn part1(parsed: &Parsed) -> usize {
    let mut map = vec![vec![false; 1000]; 1000];

    // Paint lines
    for line in parsed {
        let mut iter = line.iter();
        let mut cur = iter.next().unwrap().to_owned();
        for next in iter {
            while cur != *next {
                map[cur.0][cur.1] = true;
                creep(&mut cur.0, next.0);
                creep(&mut cur.1, next.1);
                map[cur.0][cur.1] = true;
            }
        }
    }

    let mut count = 0;
    'outer: loop {
        let mut sand = (500, 0);

        while sand.1 < 1000 - 1 {
            if !map[sand.0][sand.1 + 1] {
                sand = (sand.0, sand.1 + 1);
            } else if !map[sand.0 - 1][sand.1 + 1] {
                sand = (sand.0 - 1, sand.1 + 1)
            } else if !map[sand.0 + 1][sand.1 + 1] {
                sand = (sand.0 + 1, sand.1 + 1)
            } else {
                map[sand.0][sand.1] = true;
                count += 1;
                continue 'outer;
            }
        }
        break;
    }
    count
}

fn part2(parsed: &Parsed) -> usize {
    let mut map = vec![vec![false; 1000]; 1000];

    let floory = parsed
        .iter()
        .flat_map(|x| x.iter().map(|(_, y)| y))
        .max()
        .unwrap()
        + 2;
    let floor = vec![(0, floory), (999, floory)];
    let parsed = parsed.iter().chain(once(&floor));

    // Paint lines
    for line in parsed {
        let mut iter = line.iter();
        let mut cur = iter.next().unwrap().to_owned();
        for next in iter {
            while cur != *next {
                map[cur.0][cur.1] = true;
                creep(&mut cur.0, next.0);
                creep(&mut cur.1, next.1);
                map[cur.0][cur.1] = true;
            }
        }
    }

    let mut count = 0;
    'outer: loop {
        let mut sand = (500, 0);

        while !map[500][0] {
            if !map[sand.0][sand.1 + 1] {
                sand = (sand.0, sand.1 + 1);
            } else if !map[sand.0 - 1][sand.1 + 1] {
                sand = (sand.0 - 1, sand.1 + 1)
            } else if !map[sand.0 + 1][sand.1 + 1] {
                sand = (sand.0 + 1, sand.1 + 1)
            } else {
                map[sand.0][sand.1] = true;
                count += 1;
                continue 'outer;
            }
        }
        break;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 24);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 93);
    }
}
