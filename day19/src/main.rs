use std::{
    cmp::max,
    collections::HashSet,
    io::{self, BufRead},
    ops::Deref,
};

use sscanf::sscanf;

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore: i32,
    clay_ore: i32,
    obsidian_ore: i32,
    obsidian_clay: i32,
    geode_ore: i32,
    geode_obsidian: i32,
}

impl Blueprint {
    fn quality(&self, minutes: i32) -> i32 {
        #[derive(Default, Debug, Clone, PartialEq, Hash, Eq)]
        struct Node {
            t: i32,
            ore: i32,
            clay: i32,
            obsidian: i32,
            geode: i32,
            r_ore: i32,
            r_clay: i32,
            r_obsidian: i32,
            r_geode: i32,
        }

        let mut start = Node::default();
        start.r_ore = 1;

        let max_ore = max(self.ore, max(self.geode_ore, max(self.clay_ore, self.obsidian_ore)));
        let max_clay = self.obsidian_clay;
        let max_obsidian = self.geode_obsidian;

        let mut seen = HashSet::new();
        let mut q = vec![start];
        while let Some(mut it) = q.pop() {
            if seen.contains(&it) {
                continue;
            }
            seen.insert(it.clone());
            it.t += 1;
            let b_ore = it.ore >= self.ore && it.r_ore < max_ore;
            let b_clay = it.ore >= self.clay_ore && it.r_clay < max_clay;
            let b_obsidian = it.ore >= self.obsidian_ore && it.clay >= self.obsidian_clay && it.r_obsidian < max_obsidian;
            let b_geode = it.ore >= self.geode_ore && it.obsidian >= self.geode_obsidian;
            it.ore += it.r_ore;
            it.clay += it.r_clay;
            it.obsidian += it.r_obsidian;
            it.geode += it.r_geode;
            if (it.t == minutes) {
                seen.insert(it);
                continue;
            }
            if b_geode {
                let mut it = it.clone();
                it.r_geode += 1;
                it.ore -= self.geode_ore;
                it.obsidian -= self.geode_obsidian;
                q.push(it);
            } else {
                if b_ore {
                    let mut it = it.clone();
                    it.r_ore += 1;
                    it.ore -= self.ore;
                    q.push(it);
                }
                if b_clay {
                    let mut it = it.clone();
                    it.r_clay += 1;
                    it.ore -= self.clay_ore;
                    q.push(it);
                }
                if b_obsidian {
                    let mut it = it.clone();
                    it.r_obsidian += 1;
                    it.ore -= self.obsidian_ore;
                    it.clay -= self.obsidian_clay;
                    q.push(it);
                }
                if !(b_ore && b_clay && b_obsidian) {
                    q.push(it);
                }
            }
            
        }
        let q = seen.iter()
            .filter(|&x| x.t == minutes)
            .max_by_key(|&x| (x.geode, x.obsidian, x.clay, x.ore))
            .unwrap();
        println!("{:?}: {q:?}", &self);
        q.geode
    }
}

type Parsed = Vec<Blueprint>;

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
        let (id, ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = sscanf!(line, "Blueprint {i32}: Each ore robot costs {i32} ore. Each clay robot costs {i32} ore. Each obsidian robot costs {i32} ore and {i32} clay. Each geode robot costs {i32} ore and {i32} obsidian.").unwrap();
        result.push(Blueprint {
            id,
            ore,
            clay_ore,
            obsidian_ore,
            obsidian_clay,
            geode_obsidian,
            geode_ore,
        });
    }
    result
}

fn part1(parsed: &Parsed) -> usize {
    parsed.iter().map(|x| x.id * x.quality(24)).sum::<i32>() as usize
}

fn part2(parsed: &Parsed) -> usize {
    parsed.iter().take(3).map(|x| x.quality(32) as i64).product::<i64>() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.  
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 33);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 36);
    }
}
