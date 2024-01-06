use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

struct DirNode {
    children: HashSet<String>,
    files: HashMap<String, usize>,
}

struct DirTree {
    nodes: HashMap<String, DirNode>,
}

impl DirTree {
    fn new() -> Self {
        let root = DirNode {
            children: HashSet::new(),
            files: HashMap::new(),
        };
        let mut nodes: HashMap<String, DirNode> = HashMap::new();
        nodes.insert("/".into(), root);
        DirTree { nodes }
    }

    fn size(&self, path: &str) -> usize {
        let dir = self.nodes.get(path).unwrap();

        dir.children.iter().map(
              |c| self.size(Path::new(path).join(c).to_str().unwrap())
            ).sum::<usize>()
         + dir.files.values().sum::<usize>()
    }

    fn mkdir(&mut self, cwd: &str, name: &str) {
        let path:String = Path::new(cwd).join(name).to_str().unwrap().into();
        if self.nodes.contains_key(&path) {
            return;
        }
        let dir = DirNode {
            children: HashSet::new(),
            files: HashMap::new(),
        };
        self.nodes.insert(path, dir);
        let parent = self.nodes.get_mut(cwd).unwrap();
        parent.children.insert(name.into());
    }

    fn mkfile(&mut self, cwd: &str, name: &str, size: usize) {
        let dir = self.nodes.get_mut(cwd).unwrap();
        dir.files.insert(name.into(), size);
    }

    fn get(&self, path: &str) -> &DirNode {
        self.nodes.get(path).unwrap()
    }
}

struct Chdir {
    dir: String,
}

struct InputFile {
    name: String,
    size: usize,
}

struct InputDir {
    name: String,
}

enum Input {
    Chdir(Chdir),
    File(InputFile),
    Dir(InputDir),
    Empty,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CDRE: Regex = Regex::new(r"^\$ cd (.*)").unwrap();
        }
        lazy_static! {
            static ref DRE: Regex = Regex::new(r"^dir (.*)").unwrap();
        }
        lazy_static! {
            static ref FRE: Regex = Regex::new(r"^(\d+) (.*)").unwrap();
        }
        if let Some(caps) = CDRE.captures(s) {
            Ok(Input::Chdir(Chdir {
                dir: caps.get(1).unwrap().as_str().into()
            }))
        }
        else if let Some(caps) = DRE.captures(s) {
            Ok(Input::Dir(InputDir {
                name: caps.get(1).unwrap().as_str().into()
            }))
        }
        else if let Some(caps) = FRE.captures(s) {
            Ok(Input::File(InputFile {
                size: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                name: caps.get(2).unwrap().as_str().into(),
            }))
        }
        else {
            Ok(Input::Empty)
        }
    }
}

fn build_tree(input: &[Input]) -> DirTree {
    let mut tree: DirTree = DirTree::new();
    let mut cwd:PathBuf = PathBuf::from("/");
    for row in input {
        match row {
            Input::Chdir(chdir) => {
                if chdir.dir == ".." {
                    cwd.pop();
                }
                else {
                    cwd.push(&chdir.dir);
                }
            },
            Input::File(ifile) => {
                tree.mkfile(cwd.to_str().unwrap(), &ifile.name, ifile.size);
            },
            Input::Dir(idir) => {
                tree.mkdir(cwd.to_str().unwrap(), &idir.name);
            },
            Input::Empty => {},
        }
    }
    tree
}

fn search(tree: &DirTree, dir: &str, thresh: usize, bigger: bool, results: &mut Vec<usize>) {
    let size = tree.size(dir);
    if !bigger && size <= thresh || bigger && size >= thresh {
        results.push(size);
    }
    let dirnode = tree.get(dir);
    for c in dirnode.children.iter() {
        let path:String = Path::new(&dir).join(c).to_str().unwrap().into();
        search(tree, &path, thresh, bigger, results);
    }
}

fn part1(input: &[Input]) -> usize {
    let tree = build_tree(input);
    let mut results: Vec<usize> = Vec::new();
    search(&tree, "/", 100000, false, &mut results);
    results.iter().sum()
}

fn part2(input: &[Input]) -> usize {
    let tree = build_tree(input);
    let total = tree.size("/");
    let needed = 30000000 - (70000000 - total);

    let mut results: Vec<usize> = Vec::new();
    search(&tree, "/", needed, true, &mut results);
    results.into_iter().min().unwrap()
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day07_test() {
        let input: Vec<Input> = test_input(include_str!("day07.testinput"));
        assert_eq!(part1(&input), 95437);
        assert_eq!(part2(&input), 24933642);
    }
}
