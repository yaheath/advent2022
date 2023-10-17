#[macro_use] extern crate lazy_static;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::str::FromStr;
use regex::Regex;
use advent_lib::read::read_input;

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
        DirTree { nodes: nodes }
    }

    fn size(&self, path: String) -> usize {
        let dir = self.nodes.get(&path).unwrap();

        dir.children.iter().map(
              |c| self.size(Path::new(&path).join(&c).to_str().unwrap().into())
            ).sum::<usize>()
         + dir.files.iter().map(|(_, size)| size).sum::<usize>()
    }

    fn mkdir(&mut self, cwd: &String, name: &String) {
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
        parent.children.insert(name.clone());
    }

    fn mkfile(&mut self, cwd: &String, name: &String, size: usize) {
        let dir = self.nodes.get_mut(cwd).unwrap();
        dir.files.insert(name.clone(), size);
    }

    fn get(&self, path: String) -> &DirNode {
        self.nodes.get(&path).unwrap()
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

fn build_tree(input: &Vec<Input>) -> DirTree {
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
                tree.mkfile(&cwd.to_str().unwrap().into(), &ifile.name, ifile.size);
            },
            Input::Dir(idir) => {
                tree.mkdir(&cwd.to_str().unwrap().into(), &idir.name);
            },
            Input::Empty => {},
        }
    }
    tree
}

fn search(tree: &DirTree, dir: String, thresh: usize, bigger: bool, results: &mut Vec<usize>) {
    let size = tree.size(dir.clone());
    if !bigger && size <= thresh || bigger && size >= thresh {
        results.push(size);
    }
    let dirnode = tree.get(dir.clone());
    for c in dirnode.children.iter() {
        let path:String = Path::new(&dir).join(&c).to_str().unwrap().into();
        search(tree, path, thresh, bigger, results);
    }
}

fn part1(input: &Vec<Input>) {
    let tree = build_tree(input);
    let mut results: Vec<usize> = Vec::new();
    search(&tree, "/".into(), 100000, false, &mut results);
    let value:usize = results.iter().sum();
    println!("Part 1: {}", value);
}

fn part2(input: &Vec<Input>) {
    let tree = build_tree(input);
    let total = tree.size("/".into());
    let needed = 30000000 - (70000000 - total);

    let mut results: Vec<usize> = Vec::new();
    search(&tree, "/".into(), needed, true, &mut results);
    results.sort();
    println!("Part 2: {}", results[0]);
}

fn main() {
    let input: Vec<Input> = read_input::<Input>();
    part1(&input);
    part2(&input);
}
