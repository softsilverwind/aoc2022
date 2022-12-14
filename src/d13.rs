use std::{
    io::{self, BufRead, BufReader},
    str::FromStr,
    cmp::{Ord, Ordering}
};

use anyhow::{Result, bail, anyhow};
use itertools::Itertools;
use serde_json::Value;

#[derive(Clone, PartialEq, Eq)]
enum Packet
{
    Number(i64),
    Array(Vec<Packet>)
}

impl Packet
{
    fn from_json(json: Value) -> Result<Packet>
    {
        Ok(match json {
            Value::Number(n) => Packet::Number(n.as_i64().ok_or(anyhow!("Invalid number"))?),
            Value::Array(arr) => {
                let res: Result<Vec<Packet>, _> = arr.into_iter().map(|x| Packet::from_json(x)).collect();
                Packet::Array(res?)
            }
            _ => bail!("Invalid input")
        })
    }
}

impl FromStr for Packet
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Packet>
    {
        Packet::from_json(serde_json::from_str(s)?)
    }
}

fn cmp_slices(this: &[Packet], that: &[Packet]) -> Ordering
{
    match (this, that) {
        ([], []) => Ordering::Equal,
        ([], _) => Ordering::Less,
        (_, []) => Ordering::Greater,
        ([h1, t1@..], [h2, t2@..]) => h1.cmp(h2).then(cmp_slices(t1, t2))
    }
}

impl Ord for Packet
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match (self, other) {
            (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(n2),
            (Packet::Number(n), Packet::Array(arr)) => cmp_slices(&[Packet::Number(*n)], arr),
            (Packet::Array(arr), Packet::Number(n)) => cmp_slices(arr, &[Packet::Number(*n)]),
            (Packet::Array(arr1), Packet::Array(arr2)) => cmp_slices(arr1, arr2)
        }
    }
}

impl PartialOrd for Packet
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

pub fn simple()
{
    let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .batching(|iter| {
            let Some(line1) = iter.next() else { return None };
            let Some(line2) = iter.next() else { return None };
            iter.next();

            let p1 = Packet::from_str(&line1).unwrap();
            let p2 = Packet::from_str(&line2).unwrap();
            Some((p1, p2))
        })
        .enumerate()
        .map(|(i, (p1, p2))|
            if p1 < p2 {
                i+1
            }
            else {
                0
            }
        )
        .sum::<usize>();

    println!("{}", ans);
}

pub fn complex()
{
    let mut packets: Vec<Packet> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .filter_map(|str| {
            Packet::from_str(&str).ok()
        })
        .collect();

    let p1 = Packet::from_str("[[2]]").unwrap();
    let p2 = Packet::from_str("[[6]]").unwrap();

    packets.push(p1.clone());
    packets.push(p2.clone());

    packets.sort();

    let i1 = packets.binary_search(&p1).unwrap() + 1;
    let i2 = packets.binary_search(&p2).unwrap() + 1;

    println!("{}", i1 * i2);
}
