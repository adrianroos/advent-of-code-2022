use std::{
    io::{self, BufRead},
    iter::repeat,
    ops::Deref,
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for line in lines {
        let line: &str = &*line;
        let mut line_vec = vec![];
        for c in line.trim().chars() {
            line_vec.push(c.to_digit(10).unwrap());
        }
        result.push(line_vec)
    }
    result
}

fn clone_false(map: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    map.iter()
        .map(|x| x.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(map: &Vec<Vec<u32>>) -> usize {
    let mut visible = clone_false(map);

    for (x, line) in map.iter().enumerate() {
        for (y, &height) in line.iter().enumerate() {
            visible[x][y] = is_visible(map, x, y, height)
        }
    }

    visible.into_iter().flatten().filter(|&x| x).count()
}

fn is_visible(map: &[Vec<u32>], x: usize, y: usize, height: u32) -> bool {
    let lines_of_sight: Vec<Box<dyn Iterator<Item = _>>> = vec![
        Box::new((0..x).zip(repeat(y))),
        Box::new((x + 1..map.len()).zip(repeat(y))),
        Box::new(repeat(x).zip(0..y)),
        Box::new(repeat(x).zip(y + 1..map.first().unwrap().len())),
    ];

    for range in lines_of_sight {
        if range.map(|(x, y)| map[x][y]).all(|h| h < height) {
            return true;
        }
    }
    return false;
}

fn part2(map: &Vec<Vec<u32>>) -> usize {
    let mut max = 0;

    for (x, line) in map.iter().enumerate() {
        for (y, &height) in line.iter().enumerate() {
            max = usize::max(max, score(map, x, y, height));
        }
    }

    max
}

fn score(map: &[Vec<u32>], x: usize, y: usize, height: u32) -> usize {
    let lines_of_sight: Vec<Box<dyn Iterator<Item = _>>> = vec![
        Box::new((0..x).rev().zip(repeat(y))),
        Box::new((x + 1..map.len()).zip(repeat(y))),
        Box::new(repeat(x).zip((0..y).rev())),
        Box::new(repeat(x).zip(y + 1..map.first().unwrap().len())),
    ];

    eprint!("{y},{x} - {height}:");
    let mut score = 1;
    for range in lines_of_sight {
        let mut dist = 0;
        for (_i, h) in range.map(|(x, y)| map[x][y]).enumerate() {
            dist += 1;
            if h >= height {
                break;
            }
        }
        eprint!("{dist}");
        score *= dist;
    }
    eprintln!();
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    30373
    25512
    65332
    33549
    35390";

    fn sample_lines() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample_lines());
        assert_eq!(part1(&root), 21);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample_lines());
        assert_eq!(part2(&root), 8);
    }
}
