use std::{
    cmp::max,
    collections::HashSet,
    io::{self, BufRead},
};

use sscanf;

type Parsed = Vec<((i32, i32), (i32, i32))>;

fn dist((sx, sy): (i32, i32), (bx, by): (i32, i32)) -> i32 {
    (sx - bx).abs() + (sy - by).abs()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map, 2000000));
    println!("part2: {}", part2(&map, 4000000));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut result = Parsed::new();
    for line in lines {
        let (sx, sy, bx, by) = sscanf::sscanf!(
            line,
            "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
        )
        .unwrap();
        result.push(((sx, sy), (bx, by)));
    }
    result

    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

fn part1(parsed: &Parsed, y: i32) -> usize {
    let mut exclusions = vec![];
    let mut events = vec![];
    for &((sx, sy), (bx, by)) in parsed {
        let d = dist((sx, sy), (bx, by));
        let r = d - (sy - y).abs();
        if r > 0 {
            exclusions.push(sx - r..=sx + r);
            events.push((sx - r, true, sx - r..=sx + r));
            events.push((sx + r, false, sx - r..=sx + r));
        }
    }
    let mut count = 0;
    let mut open = HashSet::new();
    let mut prev = i32::MIN;
    events.sort_by_key(|(e, o, _)| (*e, *o));
    for (e, o, r) in events {
        print!("{e}  -  {o}  -  {r:?}");
        if !open.is_empty() && prev < e {
            print!(" add {}", e - prev);
            count += e - prev;
        } else if prev < e {
            print!(" add {}", 1);
            count += 1;
            prev = e + 1;
        }
        println!();
        if o {
            open.insert(r);
        } else {
            open.remove(&r);
        }
        prev = max(prev, e);
    }

    count as usize
}

fn part2(parsed: &Parsed, r: i32) -> usize {
    for y in 0..=r {
        let mut exclusions = vec![];
        let mut events = vec![];
        for &((sx, sy), (bx, by)) in parsed {
            let d = dist((sx, sy), (bx, by));
            let r = d - (sy - y).abs();
            if r > 0 {
                exclusions.push(sx - r..=sx + r);
                events.push((sx - r, true, sx - r..=sx + r));
                events.push((sx + r, false, sx - r..=sx + r));
            }
        }
        events.push((r + 1, true, r + 1..=i32::MAX));

        let mut open = HashSet::new();
        let mut prev = i32::MIN;
        events.sort_by_key(|(e, o, _)| (*e, *o));
        for (e, o, r) in events {
            // print!("{e}  -  {o}  -  {r:?}");
            // println!();

            if open.is_empty() && max(prev + 1, 0) < e {
                return max(prev + 1, 0) as usize * 4000000 + y as usize;
            }

            if o {
                open.insert(r);
            } else {
                open.remove(&r);
            }
            prev = e;
        }
    }
    panic!("not found");
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root, 10), 26);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root, 20), 56000011);
    }
}
