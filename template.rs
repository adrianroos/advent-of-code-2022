use std::{
    collections::{HashSet},
    io::{self, BufRead},
    ops::Deref,
};

type Parsed = Vec<()>;

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
    }
    result
   
    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

fn part1(parsed: &Parsed) -> usize {
    todo!();
}

fn part2(parsed: &Parsed) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 13);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 36);
    }
}
