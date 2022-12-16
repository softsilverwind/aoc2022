use std::{
    io::{self, BufRead, BufReader},
    collections::{HashMap, VecDeque, BTreeSet, HashSet}
};
use itertools::Itertools;
use regex::Regex;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Valve
{
    flow: i32,
    neighbours: HashMap<String, i32>
}

fn find_distance(start: &str, edges: &HashMap<String, Vec<String>>) -> HashMap<String, i32>
{
    let mut ret = HashMap::new();
    let mut front = VecDeque::new();
    front.push_back((start, 0));

    while let Some((node, dist)) = front.pop_front() {
        let prev_dist = ret.get(node).copied().unwrap_or(i32::MAX);

        if dist < prev_dist {
            ret.insert(node.to_string(), dist);
            for next in edges[node].iter() {
                front.push_back((next, dist + 1));
            }
        }
    }

    ret.remove(start);

    ret
}

fn read_input() -> HashMap<String, Valve>
{
    let regex = Regex::new(r"Valve (..) has flow rate=(\d*); tunnels? leads? to valves? (.*)").unwrap();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    let mut valves: HashMap<String, Valve> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            let cap = regex.captures(&line).unwrap();
            let vname = cap[1].to_string();
            let flow = cap[2].parse().unwrap();

            edges.insert(vname.clone(), cap[3].split(", ").map(|x| x.to_string()).collect());
        
            (vname, Valve { flow, neighbours: HashMap::new() })
        })
        .collect();

    for (vname, valve) in valves.iter_mut() {
        valve.neighbours = find_distance(vname, &edges);
    }

    valves.retain(|vname, valve| vname == "AA" || valve.flow > 0);

    let keys: HashSet<String> = valves.keys().cloned().collect();

    for valve in valves.values_mut() {
        valve.neighbours.retain(|k, _| keys.contains(k));
    }

    valves
}

fn solve(valves: &HashMap<String, Valve>, minutes: i32) -> i32
{
    let mut front: VecDeque<(&str, i32, i32, BTreeSet<&str>)> = VecDeque::new();
    front.push_back(("AA", minutes, 0, BTreeSet::new()));

    let mut visited: HashMap<(&str, BTreeSet<&str>), i32> = HashMap::new();

    let mut max_score = 0;

    while let Some((vname, minutes, score, opened)) = front.pop_front() {
        let prev_score = visited.get(&(vname, opened.clone())).copied().unwrap_or(0);

        if prev_score > score {
            continue;
        }
        visited.insert((vname, opened.clone()), score);

        max_score = max_score.max(score);

        let valve = &valves[vname];
 
        for (next, &dist) in valve.neighbours.iter() {
            let rem = minutes - dist - 1;
            if !opened.contains(&next.as_ref()) && valves.contains_key(next) && rem > 0 {
                let flow = valves[next].flow;
                let mut new_opened = opened.clone();
                new_opened.insert(next);
                front.push_back((next, rem, score + flow * rem, new_opened));
            }
        }
    }
 
    max_score
}

pub fn simple()
{
    let valves = read_input();
    let solution = solve(&valves, 30);
    println!("{solution}");
}

fn powerset<T: Clone + Ord + Copy>(items: &BTreeSet<T>) -> Vec<BTreeSet<T>>
{
    (0..=items.len())
    .flat_map(|count|
        items
        .iter()
        .copied()
        .combinations(count)
        .map(|v| v.into_iter().collect::<BTreeSet<_>>())
    )   
    .collect()
}

pub fn complex()
{
    let valves = read_input();
    let keys: BTreeSet<&str> = valves.keys().map(|x| x.as_ref()).filter(|x| *x != "AA").collect();

    let max_result = powerset(&keys).into_par_iter().map(|combination| {
        let complement: BTreeSet<&str> = keys.difference(&combination).copied().collect();

        let mut v1: HashMap<String, Valve> = combination.iter().map(|vname| (vname.to_string(), valves[*vname].clone())).collect();
        v1.insert("AA".to_string(), valves["AA"].clone());
        let mut v2: HashMap<String, Valve> = complement.iter().map(|vname| (vname.to_string(), valves[*vname].clone())).collect();
        v2.insert("AA".to_string(), valves["AA"].clone());

        solve(&v1, 26) + solve(&v2, 26)
    }).max().unwrap();

    println!("{max_result}");
}
