use std::{
    io::{self, BufRead, BufReader},
    collections::HashMap
};

use itertools::Itertools;

const CUTOFF: i32 = 100000;
const UPGRADE_SIZE: i32 = 30000000;
const FS_SIZE: i32 = 70000000;

#[derive(Debug)]
enum DirEntry
{
    Dir(HashMap<String, DirEntry>),
    File(i32)
}

impl DirEntry
{
    fn force_mut_dir(&mut self) -> &mut HashMap<String, DirEntry>
    {
        match self {
            DirEntry::Dir(x) => x,
            _ => panic!()
        }
    }
}

enum DirInput
{
    Dir(String),
    File(i32, String)
}

enum Command
{
    CD(String),
    LS
}

enum Input
{
    Command(Command),
    DirInput(DirInput)
}

fn read_input() -> Vec<Input>
{
    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            if line.starts_with("$ ls") {
                Input::Command(Command::LS)
            }
            else if line.starts_with("$ cd") {
                Input::Command(Command::CD(line.split(' ').last().unwrap().to_string()))
            }
            else if line.starts_with("dir") {
                Input::DirInput(DirInput::Dir(line.split(' ').last().unwrap().to_string()))
            }
            else {
                let (num, name) = line.split(' ').next_tuple().unwrap();
                Input::DirInput(DirInput::File(num.parse().unwrap(), name.to_string()))
            }
        })
        .collect()
}

fn parse_dir_tree(input: Vec<Input>) -> HashMap<String, DirEntry>
{
    let mut root: HashMap<String, DirEntry> = HashMap::new();

    let mut curr = &mut root;
    let mut pwd: Vec<String> = Vec::new();

    for line in input {
        match line {
            Input::Command(Command::CD(name)) if name == "/" => {
                curr = &mut root;
                pwd.clear();
            }
            Input::Command(Command::CD(name)) if name == ".." => {
                curr = &mut root;
                pwd.pop();

                for part in pwd.iter() {
                    curr = curr.get_mut(part).unwrap().force_mut_dir();
                }
            }
            Input::Command(Command::CD(name)) => {
                pwd.push(name.clone());
                curr = curr.entry(name).or_insert(DirEntry::Dir(HashMap::new())).force_mut_dir();
            },
            Input::Command(Command::LS) => (),
            Input::DirInput(DirInput::Dir(name)) => {
                curr.insert(name, DirEntry::Dir(HashMap::new()));
            }
            Input::DirInput(DirInput::File(size, name)) => {
                curr.insert(name, DirEntry::File(size));
            }
        }
    }

    root
}

fn size_and_agg_size(tree: &HashMap<String, DirEntry>) -> (i32, i32)
{
    let mut size = 0;
    let mut agg_size = 0;

    for (_, val) in tree {
        match val {
            DirEntry::Dir(child) => {
                let (child_size, child_agg_size) = size_and_agg_size(child);
                size += child_size;
                agg_size += child_agg_size;
            },
            DirEntry::File(filesize) => {
                size += filesize;
            },
        }
    }

    if size <= CUTOFF {
        agg_size += size;
    }

    (size, agg_size)
}

fn find_minimal_folder(tree: &HashMap<String, DirEntry>, needed: i32) -> (i32, i32)
{
    let mut size = 0;
    let mut minimal_size = FS_SIZE;

    for (_, val) in tree {
        match val {
            DirEntry::Dir(child) => {
                let (child_size, child_minimal) = find_minimal_folder(child, needed);
                size += child_size;
                
                if (needed..minimal_size).contains(&child_minimal) {
                    minimal_size = child_minimal;
                }
            },
            DirEntry::File(filesize) => {
                size += filesize;
            },
        }
    }

    if minimal_size == FS_SIZE {
        minimal_size = size;
    }

    (size, minimal_size)
}

pub fn simple()
{
    let input = read_input();
    let root = parse_dir_tree(input);
    println!("{}", size_and_agg_size(&root).1);
}

pub fn complex()
{
    let input = read_input();
    let root = parse_dir_tree(input);
    let du = size_and_agg_size(&root).0;
    let needed = UPGRADE_SIZE - (FS_SIZE - du);
    println!("{}", find_minimal_folder(&root, needed).1);
}
