use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::Deref,
};

#[derive(Default, Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn size(&self) -> usize {
        let files: usize = self.files.values().sum();
        let dirs: usize = self.dirs.values().map(|x| x.size()).sum();
        files + dirs
    }

    fn sizes(&self, name: String) -> Vec<(String, usize)> {
        let mut sizes = self
            .dirs
            .iter()
            .flat_map(|(n, d)| d.sizes(n.to_owned()))
            .collect::<Vec<_>>();
        sizes.push((name, self.size()));
        sizes
    }
}

enum Line {
    Cd(Cd),
    Ls,
    DirEnt(DirEnt),
}

impl<T: Deref<Target = str>> From<T> for Line {
    fn from(s: T) -> Self {
        let s = &*s;
        if let Some(dir) = s.strip_prefix("$ cd ") {
            Line::Cd(match dir {
                "/" => Cd::Root,
                ".." => Cd::Up,
                path => Cd::Into(path.to_owned()),
            })
        } else if s == "$ ls" {
            Line::Ls
        } else if let Some(name) = s.strip_prefix("dir ") {
            Line::DirEnt(DirEnt::Dir(name.to_owned()))
        } else {
            let (size, name) = s.split_once(' ').unwrap();
            let size = size
                .parse::<usize>()
                .expect(format!("not a number: {s}").as_str());
            let name = name.to_owned();
            Line::DirEnt(DirEnt::File { size, name })
        }
    }
}

enum DirEnt {
    Dir(String),
    File { size: usize, name: String },
}

enum Cd {
    Root,
    Up,
    Into(String),
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let root = parse(lines);
    println!("part1: {}", part1(&root));
    println!("part2: {}", part2(&root));
}

enum ParseResult {
    Up,
    Root,
    Eof,
}

fn parse_into(
    dir: &mut Dir,
    lines: &mut impl Iterator<Item = impl Deref<Target = str>>,
) -> ParseResult {
    while let Some(line) = lines.next() {
        match Line::from(line) {
            Line::Cd(cd) => match cd {
                Cd::Root => return ParseResult::Root,
                Cd::Up => return ParseResult::Up,
                Cd::Into(path) => {
                    let mut inner = dir.dirs.entry(path).or_default();
                    match parse_into(&mut inner, lines) {
                        ParseResult::Root => return ParseResult::Root,
                        ParseResult::Eof => return ParseResult::Eof,
                        ParseResult::Up => (),
                    }
                }
            },
            Line::Ls => (),
            Line::DirEnt(d) => match d {
                DirEnt::Dir(_) => (),
                DirEnt::File { size, name } => {
                    dir.files.insert(name, size);
                }
            },
        }
    }
    ParseResult::Eof
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Dir {
    let mut root = Dir::default();
    let mut lines = lines;
    loop {
        if let ParseResult::Eof = parse_into(&mut root, &mut lines) {
            return root;
        }
    }
}

fn part1(root: &Dir) -> usize {
    root.sizes("".to_owned())
        .into_iter()
        .map(|x| x.1)
        .filter(|&x| x <= 100000)
        .sum::<usize>()
}

fn part2(root: &Dir) -> usize {
    let needed = root.size() - (70000000 - 30000000);
    root.sizes("/".to_owned())
        .into_iter()
        .filter(|x| x.1 >= needed)
        .min_by_key(|x| x.1)
        .unwrap()
        .1
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";

    fn sample_lines() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample_lines());
        assert_eq!(part1(&root), 95437);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample_lines());
        assert_eq!(part2(&root), 24933642);
    }
}
