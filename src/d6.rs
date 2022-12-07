use std::{
    io::{self, BufReader},
    collections::HashSet
};

use itertools::Itertools;

pub fn simple()
{
    println!("{}",
        io::read_to_string(BufReader::new(io::stdin())).unwrap()
            .chars()
            .tuple_windows()
            .enumerate()
            .find(|(_, (c1, c2, c3, c4))| [c1, c2, c3, c4].into_iter().copied().collect::<HashSet<_>>().len() == 4)
            .unwrap()
            .0
            + 4
    );
}

pub fn complex()
{
    let chars: Vec<char> = io::read_to_string(BufReader::new(io::stdin())).unwrap().chars().collect();

    for i in 0.. {
        if chars[i..i+14].into_iter().copied().collect::<HashSet<_>>().len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}
