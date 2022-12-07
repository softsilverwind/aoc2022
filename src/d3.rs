use std::{
    io::{self, BufRead, BufReader},
    collections::HashSet
};

use itertools::Itertools;

pub fn simple()
{
    let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first_set: HashSet<u8> = first.as_bytes().iter().copied().collect();
            let second_set: HashSet<u8> = second.as_bytes().iter().copied().collect();
            let common = *first_set.intersection(&second_set).next().unwrap();

            let score = if common >= 'a' as u8 && common <= 'z' as u8 {
                common - 'a' as u8 + 1
            }
            else {
                common - 'A' as u8 + 27
            };
            
            score as i32
        })
        .sum::<i32>();

    println!("{}", ans);
}

pub fn complex()
{
        let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .tuples()
        .map(|(first, second, third)| {
            let first_set: HashSet<u8> = first.as_bytes().iter().copied().collect();
            let second_set: HashSet<u8> = second.as_bytes().iter().copied().collect();
            let third_set: HashSet<u8> = third.as_bytes().iter().copied().collect();

            let fs = first_set.intersection(&second_set).copied().collect::<HashSet<_>>();
            let common = *fs.intersection(&third_set).next().unwrap();

            let score = if common >= 'a' as u8 && common <= 'z' as u8 {
                common - 'a' as u8 + 1
            }
            else {
                common - 'A' as u8 + 27
            };

            score as i32
        })
        .sum::<i32>();

    println!("{}", ans);
}
