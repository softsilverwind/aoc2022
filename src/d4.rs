use std::io::{self, BufRead, BufReader};

use itertools::Itertools;
use regex::Regex;

pub fn simple()
{
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            let (s1, e1, s2, e2) = re.captures(&line).unwrap().iter().skip(1).map(|cap| cap.unwrap().as_str().parse::<i32>().unwrap()).next_tuple().unwrap();

            let r1 = s1..=e1;
            let r2 = s2..=e2;

            if r1.contains(&s2) && r1.contains(&e2) || r2.contains(&s1) && r2.contains(&e1) { 1 } else { 0 }
        })
        .sum::<i32>();

    println!("{}", ans);
}

pub fn complex()
{
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            let (s1, e1, s2, e2) = re.captures(&line).unwrap().iter().skip(1).map(|cap| cap.unwrap().as_str().parse::<i32>().unwrap()).next_tuple().unwrap();

            let r1 = s1..=e1;
            let r2 = s2..=e2;

            if r1.contains(&s2) || r1.contains(&e2) || r2.contains(&s1) || r2.contains(&e1) { 1 } else { 0 }
        })
        .sum::<i32>();

    println!("{}", ans);
}
