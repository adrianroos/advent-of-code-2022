use std::io::{self, BufRead};

type Parsed = Vec<i32>;

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
        result.push(line.parse::<i32>().unwrap());
    }
    result

    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

fn part1(parsed: &Parsed) -> i32 {
    let mut v = parsed.iter().cloned().enumerate().collect::<Vec<_>>();
    // println!("{:?}", v.iter().map(|x| x.1).collect::<Vec<_>>());

    for i in 0..v.len() {
        let i = v.iter().position(|x| x.0 == i).unwrap();
        let r = v.remove(i);
        let newi = i as i32 + r.1;
        v.insert((newi.rem_euclid(v.len() as i32)) as usize, r);
        // println!("{:?}", v.iter().map(|x| x.1).collect::<Vec<_>>());
    }

    let zero = v.iter().position(|x| x.1 == 0).unwrap();

    v[(zero + 1000) % v.len()].1 + v[(zero + 2000) % v.len()].1 + v[(zero + 3000) % v.len()].1
}

fn part2(parsed: &Parsed) -> i64 {
    let mut v = parsed
        .iter()
        .cloned()
        .map(|x| x as i64 * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    // println!("{:?}", v.iter().map(|x| x.1).collect::<Vec<_>>());

    for _ in 0..10 {
        for i in 0..v.len() {
            let i = v.iter().position(|x| x.0 == i).unwrap();
            let r = v.remove(i);
            let newi = i as i64 + r.1;
            v.insert((newi.rem_euclid(v.len() as i64)) as usize, r);
            // println!("{:?}", v.iter().map(|x| x.1).collect::<Vec<_>>());
        }
    }

    let zero = v.iter().position(|x| x.1 == 0).unwrap();

    v[(zero + 1000) % v.len()].1 + v[(zero + 2000) % v.len()].1 + v[(zero + 3000) % v.len()].1
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    1
    2
    -3
    3
    -2
    0
    4";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 3);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 1623178306);
    }
}
