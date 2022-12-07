use std::io::{self, BufRead, BufReader};

use itertools::Itertools;
use regex::Regex;

pub fn process(complex: bool)
{
    let mut init: Vec<Vec<char>> = Vec::new();
    let mut lines = BufReader::new(io::stdin()).lines();
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with(" 1") { break; }

        init.push(line.chars().chunks(4).into_iter().map(|mut chunk| chunk.nth(1).unwrap()).collect::<Vec<_>>());
    }

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); init[0].len()];

    for line in init {
        for (i, char) in line.into_iter().enumerate() {
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let _ = lines.next();

    let commands: Vec<(usize, usize, usize)> = lines
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            re.captures(&line).unwrap().iter().skip(1).map(|str| str.unwrap().as_str().parse::<usize>().unwrap()).next_tuple().unwrap()
        })
        .collect();

    if !complex {
        for (num, from, to) in commands {
            for _ in 0..num {
                let elem = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(elem);
            }
        }
    }
    else {
        for (num, from, to) in commands {
            let first_elem = stacks[from - 1].len() - num;
            let elems: Vec<char> = stacks[from - 1].drain(first_elem..).collect();
            stacks[to - 1].extend(elems);
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

pub fn simple() { process(false); }
pub fn complex() { process(true); }
