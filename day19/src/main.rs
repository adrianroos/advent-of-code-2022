use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
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
        impl Node {
            fn advance(self: &mut Self, dt: i32) {
                self.t += dt;
                self.ore += self.r_ore * dt;
                self.clay += self.r_clay * dt;
                self.obsidian += self.r_obsidian * dt;
                self.geode += self.r_geode * dt;
            }
        }

        let mut start = Node::default();
        start.r_ore = 1;

        let max_ore = max(self.ore, max(self.geode_ore, max(self.clay_ore, self.obsidian_ore)));
        let max_clay = self.obsidian_clay;
        let max_obsidian = self.geode_obsidian;

        let mut seen = HashSet::new();
        let mut q = vec![start];
        let mut best = 0;
        while let Some(it) = q.pop() {
            if seen.contains(&it) {
                continue;
            }
            seen.insert(it.clone());
            if it.t == minutes {
                best = max(best, it.geode)
            }
            if it.t >= minutes {
                continue;
            }
            if it.geode + (minutes - it.t) * it.r_geode + (minutes - it.t) * (minutes - it.t + 1) / 2 < best {
                continue;
            }
            if it.r_obsidian > 0 {
                let missing_ore = self.geode_ore - it.ore;
                let missing_obsidian = self.geode_obsidian - it.obsidian;
                let dt = max(0, max((missing_ore + it.r_ore - 1) / it.r_ore, (missing_obsidian + it.r_obsidian - 1) / it.r_obsidian)) + 1;
                assert!(dt >= 1);
                let mut next = it.clone();
                next.advance(dt);
                next.r_geode += 1;
                next.ore -= self.geode_ore;
                next.obsidian -= self.geode_obsidian;
                if next.t <= minutes {
                    q.push(next);
                }
            }
            if it.r_clay > 0 && it.r_obsidian < max_obsidian {
                let missing_ore = self.obsidian_ore - it.ore;
                let missing_clay = self.obsidian_clay - it.clay;
                let dt = max(0, max((missing_ore + it.r_ore - 1) / it.r_ore, (missing_clay + it.r_clay - 1) / it.r_clay)) + 1;
                assert!(dt >= 1);
                let mut next = it.clone();
                next.advance(dt);
                next.r_obsidian += 1;
                next.ore -= self.obsidian_ore;
                next.clay -= self.obsidian_clay;
                if next.t <= minutes {
                    q.push(next);
                }
            }
            if it.r_clay < max_clay {
                let missing_ore = self.clay_ore - it.ore;
                let dt = max(0, (missing_ore + it.r_ore - 1) / it.r_ore) + 1;
                assert!(dt >= 1);
                let mut next = it.clone();
                next.advance(dt);
                next.r_clay += 1;
                next.ore -= self.clay_ore;
                if next.t <= minutes {
                    q.push(next);
                }
            }
            if it.r_ore < max_ore {
                let missing_ore = self.ore - it.ore;
                let dt = max(0, (missing_ore + it.r_ore - 1) / it.r_ore) + 1;
                assert!(dt >= 1);
                let mut next = it.clone();
                next.advance(dt);
                next.r_ore += 1;
                next.ore -= self.ore;
                if next.t <= minutes {
                    q.push(next);
                }
            }
            {
                let mut next = it;
                next.advance(minutes - next.t);
                q.push(next);
            }
        }
        let q = seen.iter()
            .filter(|&x| x.t == minutes)
            .max_by_key(|&x| (x.geode, x.obsidian, x.clay, x.ore))
            .unwrap();
        println!("{:?}: {q:?}, best: {best}", &self);
        q.geode;
        best
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
        assert_eq!(part2(&root), 56 * 62);
    }
}
