use std::{
    io::{self, BufRead, BufReader},
    collections::HashSet
};
use itertools::Itertools;
use regex::Regex;

type Link = ((i32, i32), (i32, i32));

fn read_input() -> Vec<Link>
{
    let regex = Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)").unwrap();

    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            let cap = regex.captures(&line).unwrap();

            let (x1, y1, x2, y2) = cap.iter().skip(1).map(|x| x.unwrap().as_str().parse::<i32>().unwrap()).next_tuple().unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect()
}

pub fn simple()
{
    const LINE: i32 = 2000000;

    let mut ans: HashSet<i32> = HashSet::new();
    let mut beacons: HashSet<i32> = HashSet::new();

    for ((x1, y1), (x2, y2)) in read_input() {
        let dist = (x2 - x1).abs() + (y2 - y1).abs();
        if y2 == LINE { beacons.insert(x2); }

        let rem = dist - (LINE - y1).abs();

        for i in -rem + x1 ..= rem + x1 {
            ans.insert(i);
        }
    }

    println!("{}", ans.len() - beacons.len());
}

pub fn complex()
{
    const MAX_LINE: i32 = 4000000;

    let input = read_input();

    for line in 0..MAX_LINE {
        let mut ranges: Vec<(i32, i32)> = Vec::new();

        for &((x1, y1), (x2, y2)) in &input {
            let dist = (x2 - x1).abs() + (y2 - y1).abs();

            let rem = dist - (line - y1).abs();

            if rem >= 0 {
                ranges.push(((-rem + x1).clamp(0, MAX_LINE), (rem + x1).clamp(0, MAX_LINE)));
            }
        }

        if ranges.len() < 2 {
            continue;
        }

        let mut candidates: HashSet<i32> = HashSet::new();

        for ((_, e1), (s2, _)) in ranges.iter().tuple_combinations() {
            if s2 - e1 == 2 {
                candidates.insert(s2 - 1);
            }
        }

        for (s, e) in ranges {
            candidates.retain(|&candidate| candidate < s || candidate > e);
        }

        if candidates.len() > 0 {
            let x = candidates.into_iter().next().unwrap();
            println!("{}", x as i64 * 4000000 + line as i64);
            break;
        }
    }
}
