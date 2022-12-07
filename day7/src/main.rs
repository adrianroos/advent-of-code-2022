use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead},
    ops::Deref,
    rc::Rc,
};

#[derive(Default, Debug)]
struct Dir {
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn size(&self) -> usize {
        let files: usize = self.files.values().sum();
        let dirs: usize = self.dirs.values().map(|x| x.borrow().size()).sum();
        files + dirs
    }

    fn sizes(&self, name: String) -> Vec<(String, usize)> {
        let mut sizes = self
            .dirs
            .iter()
            .flat_map(|(n, d)| d.borrow().sizes(n.to_owned()))
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

impl From<&str> for Line {
    fn from(s: &str) -> Self {
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

fn parse<'a, T: Deref<Target = str>>(lines: impl Iterator<Item = T>) -> Dir {
    let root = Rc::new(RefCell::new(Dir::default()));
    let mut stack = Vec::new();
    let mut cur = Rc::clone(&root);

    for line in lines {
        match Line::from(&*line) {
            Line::Cd(cd) => match cd {
                Cd::Root => {
                    stack.clear();
                    cur = root.clone();
                }
                Cd::Up => {
                    cur = stack.pop().unwrap_or_else(|| Rc::clone(&root));
                }
                Cd::Into(path) => {
                    let dir = {
                        let mut cur_mut = cur.borrow_mut();
                        let dir = cur_mut.dirs.entry(path).or_default();
                        Rc::clone(dir)
                    };

                    stack.push(cur);
                    cur = dir;
                }
            },
            Line::Ls => (),
            Line::DirEnt(d) => match d {
                DirEnt::Dir(_) => (),
                DirEnt::File { size, name } => {
                    cur.borrow_mut().files.insert(name, size);
                }
            },
        }
    }
    stack.clear();
    drop(cur);
    Rc::try_unwrap(root).unwrap().into_inner()
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
