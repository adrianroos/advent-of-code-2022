use core::panic;
use std::{
    io::{self, BufRead},
    iter::{once, Peekable},
    ops::Deref,
};

#[derive(Debug)]
enum ListOrItem {
    List(Vec<ListOrItem>),
    Item(i32),
}

impl ListOrItem {
    fn parse(iter: &mut Peekable<impl Iterator<Item = char>>) -> ListOrItem {
        match iter.next() {
            Some('[') => {
                let mut res = Vec::new();
                while let Some(&c) = iter.peek() {
                    if c == ']' {
                        iter.next();
                        break;
                    } else if c == ',' {
                        iter.next();
                    }
                    res.push(Self::parse(iter));
                }
                return ListOrItem::List(res);
            }
            Some(c) if c.is_digit(10) => {
                let mut res = c.to_digit(10).unwrap();
                while let Some(d) = iter.peek().and_then(|x| x.to_digit(10)) {
                    res *= 10;
                    res += d;
                    iter.next();
                }
                ListOrItem::Item(res as i32)
            }
            c => panic!("Unexpected: {c:?}"),
        }
    }
}

impl PartialEq for ListOrItem {
    fn eq(&self, other: &Self) -> bool {
        if let Some(ord) = self.partial_cmp(other) {
            ord.is_eq()
        } else {
            false
        }
    }
}

impl Eq for ListOrItem {}

impl Ord for ListOrItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ListOrItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            &ListOrItem::Item(i) => match other {
                &ListOrItem::List(ref ol) => vec![ListOrItem::Item(i)].partial_cmp(&ol),
                &ListOrItem::Item(oi) => i.partial_cmp(&oi),
            },
            &ListOrItem::List(ref l) => match other {
                &ListOrItem::List(ref ol) => l.partial_cmp(ol),
                &ListOrItem::Item(oi) => l.partial_cmp(&vec![ListOrItem::Item(oi)]),
            },
        }
    }
}

type Parsed = Vec<(ListOrItem, ListOrItem)>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Parsed {
    let mut result = Parsed::new();
    let mut lines = lines.peekable();
    while let Some(_) = lines.peek() {
        let one = lines.next().unwrap();
        let mut one = one.chars().peekable();
        let one = ListOrItem::parse(&mut one);
        let two = lines.next().unwrap();
        let mut two = two.chars().peekable();
        let two = ListOrItem::parse(&mut two);
        result.push((one, two));
        lines.next();
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
    parsed
        .iter()
        .enumerate()
        .filter(|(_, (x, y))| x < y)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(parsed: &Parsed) -> usize {
    let extra = parse(
        "\
    [[2]]
    [[6]]
    "
        .lines()
        .map(|l| l.trim()),
    );
    let mut signals = parsed
        .iter()
        .chain(extra.iter())
        .flat_map(|(x, y)| once(x).chain(once(y)))
        .collect::<Vec<_>>();
    signals.sort();
    let extra = extra.first().unwrap();
    let (x, y) = (
        signals.iter().position(|&x| *x == extra.0).unwrap(),
        signals.iter().position(|&x| *x == extra.1).unwrap(),
    );
    (x + 1) * (y + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    [1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    fn sample() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        for l in &root {
            println!("x: {l:?}");
        }
        assert_eq!(part1(&root), 13);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 140);
    }
}
