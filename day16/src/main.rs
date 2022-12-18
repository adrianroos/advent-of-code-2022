use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, BufRead},
    mem::swap,
    ops::Deref, cmp::max,
};

use sscanf;

type Parsed = HashMap<String, (u32, Vec<String>)>;

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
        let (v, r, _, o) = sscanf::sscanf!(line, "Valve {String:/[^ ]+/} has flow rate={u32}; {str:/tunnels? leads? to valves?/} {String}").expect(&format!("failed on {line}"));
        let o = o.split(", ").map(|x| x.to_owned()).collect::<Vec<_>>();
        result.insert(v, (r, o));
    }
    result

    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

fn part1(parsed: &Parsed) -> u32 {
    // state: (pos, opened, pressure)

    let start = ("AA".to_owned(), Vec::<String>::new(), 0);
    let mut d = HashMap::new();
    let mut q = VecDeque::new();
    d.insert(start.clone(), 0);
    q.push_back(start);

    while let Some(state) = q.pop_front() {
        let (ref pos, ref valves, pres) = state;
        let dcur = d[&state];
        if dcur >= 30 {
            break;
        }
        for n in parsed[pos].1.iter() {
            let ns = (n.to_owned(), valves.clone(), pres);
            if d.contains_key(&ns) {
                continue;
            }
            d.insert(ns.clone(), dcur + 1);
            q.push_back(ns);
        }
        let rate = parsed[pos].0;
        if rate != 0 && !valves.contains(pos) {
            let mut ns = (pos.to_owned(), valves.to_owned(), pres + rate * (29 - dcur));
            ns.1.push(pos.to_owned());
            ns.1.sort();
            d.insert(ns.clone(), dcur + 1);
            q.push_back(ns);
        }
    }

    d.iter()
        .map(|((_pos, _valves, pres), _dist)| *pres)
        .max()
        .unwrap()
}

type State = (i32, i32, u64, u32);

fn remap(parsed: &Parsed) -> HashMap<i32, (u32, Vec<i32>)> {
    let mut remap = HashMap::new();
    remap.insert("AA", 0);
    let mut cur = 0;
    let mut result = HashMap::new();
    for (p, (rate, ns)) in parsed {
        let p = *remap.entry(p).or_insert_with(|| {
            cur += 1;
            cur
        });
        let mut nns = Vec::new();
        for n in ns {
            nns.push(*remap.entry(n).or_insert_with(|| {
                cur += 1;
                cur
            }));
        }
        result.insert(p, (*rate, nns));
    }
    result
}

fn part2(parsed: &Parsed) -> u32 {
    // state: (pos, opened, pressure)
    let parsed = remap(parsed);
    let parsed = &parsed;

    let start = (0, 0u64, 0);
    let mut d = HashMap::new();
    let mut q = VecDeque::new();
    d.insert(start.clone(), 4);
    q.push_back(start);

    while let Some(state) = q.pop_front() {
        let (pos, valves, pres) = state;
        let dcur = d[&state];
        if dcur >= 30 {
            break;
        }
        for &n in parsed[&pos].1.iter() {
            let ns = (n, valves, pres);
            if d.contains_key(&ns) {
                continue;
            }
            d.insert(ns, dcur + 1);
            q.push_back(ns);
        }
        let rate = parsed[&pos].0;
        if rate != 0 && (valves & (1 << pos)) == 0 {
            let ns = (pos, valves | (1 << pos), pres + rate * (29 - dcur));
            d.insert(ns, dcur + 1);
            q.push_back(ns);
        }
    }

    println!("done");

    let mut best_by_valves = HashMap::<u64, u32>::new();
    for (_pos, valves, pres) in d.keys() {
        let e = best_by_valves.entry(*valves).or_default();
        *e = max(*e, *pres);
    }

    best_by_valves.iter()
        .flat_map(|(&valves, &pres)| {
            best_by_valves.iter()
                .filter(move |(&e_valves, _)| (e_valves & valves) == 0)
                .map(move |(_, e_pres)| e_pres + pres)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 1651);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 1707);
    }
}
