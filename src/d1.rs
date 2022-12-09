use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

pub fn simple()
{
    let ans: i32 = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .batching(|iter| {
            let mut sum = 0;
            
            loop {
                let Some(next) = iter.next() else {
                    if sum == 0 { return None } else { break }
                };
                let Ok(val) = next.parse::<i32>() else { break };
                sum += val;
            }
            Some(sum)
        })
        .max()
        .unwrap();

    println!("{}", ans);
}


pub fn complex()
{
    let ans: i32 = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .batching(|iter| {
            let mut sum = 0;
            
            loop {
                let Some(next) = iter.next() else {
                    if sum == 0 { return None } else { break }
                };
                let Ok(val) = next.parse::<i32>() else { break };
                sum += val;
            }
            Some(sum)
        })
        .sorted_by_key(|val| -val)
        .take(3)
        .sum::<i32>();

    println!("{}", ans);
}
