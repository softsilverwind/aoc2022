use std::{
    io::{self, BufRead, BufReader},
    collections::{HashSet, HashMap}, convert::identity
};

use derive_more::{Sub, SubAssign, Add, Div, Mul};
use regex::Regex;

#[derive(Add, Sub, Div, Mul, SubAssign, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)] struct Ore(i32);
#[derive(Add, Sub, Div, Mul, SubAssign, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)] struct Clay(i32);
#[derive(Add, Sub, Div, Mul, SubAssign, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)] struct Obsidian(i32);
#[derive(Add, Sub, Div, Mul, SubAssign, Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)] struct Geode(i32);

#[derive(Debug)]
struct Blueprint
{
    index: i32,
    ore_robot: Ore,
    clay_robot: Ore,
    obsidian_robot: (Ore, Clay),
    geode_robot: (Ore, Obsidian)    
}

#[derive(Hash, Eq, PartialEq, Default, Debug, Clone)]
struct State
{
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    geode_robot: i32,
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode
}

impl State
{
    fn new() -> State
    {
        State { ore_robot: 1, ..Default::default() }
    }

    fn mine(mut self) -> State
    {
        self.ore.0 += self.ore_robot;
        self.clay.0 += self.clay_robot;
        self.obsidian.0 += self.obsidian_robot;
        self.geode.0 += self.geode_robot;
        self
    }

    fn add_ore_robot(&self, blueprint: &Blueprint) -> Option<State>
    {
        let mut ret = self.clone();
        if self.ore < blueprint.ore_robot {
            return None;
        }

        ret.ore -= blueprint.ore_robot;
        ret = ret.mine();
        ret.ore_robot += 1;
        Some(ret)
    }

    fn add_clay_robot(&self, blueprint: &Blueprint) -> Option<State>
    {
        let mut ret = self.clone();
        if self.ore < blueprint.clay_robot {
            return None;
        }

        ret.ore -= blueprint.clay_robot;
        ret = ret.mine();
        ret.clay_robot += 1;
        Some(ret)
    }

    fn add_obsidian_robot(&self, blueprint: &Blueprint) -> Option<State>
    {
        let mut ret = self.clone();
        if self.ore < blueprint.obsidian_robot.0 || self.clay < blueprint.obsidian_robot.1 {
            return None;
        }

        ret.ore -= blueprint.obsidian_robot.0;
        ret.clay -= blueprint.obsidian_robot.1;

        ret = ret.mine();
        ret.obsidian_robot += 1;
        Some(ret)
    }

    fn add_geode_robot(&self, blueprint: &Blueprint) -> Option<State>
    {
        let mut ret = self.clone();
        if self.ore < blueprint.geode_robot.0 || self.obsidian < blueprint.geode_robot.1 {
            return None;
        }

        ret.ore -= blueprint.geode_robot.0;
        ret.obsidian -= blueprint.geode_robot.1;

        ret = ret.mine();
        ret.geode_robot += 1;
        Some(ret)
    }

    fn next(self, blueprint: &Blueprint) -> impl Iterator<Item=State>
    {
        [
            self.add_ore_robot(blueprint),
            self.add_clay_robot(blueprint),
            self.add_obsidian_robot(blueprint),
            self.add_geode_robot(blueprint)
        ]
        .into_iter()
        .filter_map(identity)
        .chain(std::iter::once(self.mine()))
        .rev()
    }
}

fn read_blueprints() -> Vec<Blueprint>
{
    let regex = Regex::new(r"(?x)
        Blueprint (\d*):
        Each ore robot costs (\d*) ore.
        Each clay robot costs (\d*) ore.
        Each obsidian robot costs (\d*) ore and (\d*) clay.
        Each geode robot costs (\d*) ore and (\d*) obsidian.
    ").unwrap();

    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            let whiteless = line.replace(" ", "");
            let cap: Vec<i32> = regex.captures(&whiteless).unwrap()
                .iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse().unwrap())
                .collect();

            Blueprint {
                index: cap[0],
                ore_robot: Ore(cap[1]),
                clay_robot: Ore(cap[2]),
                obsidian_robot: (Ore(cap[3]), Clay(cap[4])),
                geode_robot: (Ore(cap[5]), Obsidian(cap[6]))
            }
        })
        .collect()
}

fn _prune_blueprints(blueprints: &mut Vec<Blueprint>)
{
    let mut deletes = HashSet::new();

    for i in 0..blueprints.len() {
        for j in 0..blueprints.len() {
            if i == j { continue; }

            if blueprints[i].ore_robot <= blueprints[j].ore_robot
                && blueprints[i].clay_robot <= blueprints[j].clay_robot
                && blueprints[i].obsidian_robot.0 <= blueprints[j].obsidian_robot.0
                && blueprints[i].obsidian_robot.1 <= blueprints[j].obsidian_robot.1
                && blueprints[i].geode_robot.0 <= blueprints[j].geode_robot.0
                && blueprints[i].geode_robot.1 <= blueprints[j].geode_robot.1 {
                deletes.insert(blueprints[j].index);
            }
        }
    }

    dbg!(&deletes);

    blueprints.retain(|x| !deletes.contains(&x.index));
}

fn solve(blueprints: &[Blueprint], steps: i32, backtracking: bool) -> Vec<Geode>
{
    let mut ret = Vec::new();

    for blueprint in blueprints {
        let mut front: Vec<(usize, i32, State)> = vec![(usize::MAX, steps, State::new())];
        let mut best = Geode(0);
        let mut best_state = 0;
        let mut states = Vec::new();
        let mut visited = HashSet::new();

        while let Some((parent, steps, state)) = front.pop() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());
            if backtracking {
                states.push((parent, state.clone()));
            }

            if steps == 0 {
                if state.geode > best {
                    best = state.geode;
                    best_state = states.len().wrapping_sub(1);
                }
                continue;
            }

            front.extend(state.next(&blueprint).map(|nstate| (states.len().wrapping_sub(1), steps - 1, nstate)));
        }

        ret.push(best);
        println!("Index {} yielded {:?}", blueprint.index, best);

        if backtracking {
            println!("--Backtracking--");

            let mut index = best_state;
            while let Some((parent, state)) = states.get(index) {
                println!("{state:?}");
                index = *parent;
            }
        }
    }

    ret
}

pub fn simple()
{
    let blueprints = read_blueprints();

    let ans: i32 = solve(&blueprints, 24, false)
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i as i32 + 1) * x.0)
        .sum::<i32>();

    println!("{ans}");
}

type DPState = (i32, i32, i32, i32, i32, Ore, Clay, Obsidian, Geode);

fn solve_rec(state: DPState, blueprint: &Blueprint, memo: &mut HashMap<DPState, Geode>, statistics: &mut (i32, i32)) -> Geode
{
    let (steps, ro, rc, rob, rg, mut o, c, ob, g) = state;

    let max_ore = blueprint.ore_robot.max(blueprint.clay_robot).max(blueprint.obsidian_robot.0).max(blueprint.geode_robot.0) * 2;
    let max_ro = blueprint.ore_robot.max(blueprint.clay_robot).max(blueprint.obsidian_robot.0).max(blueprint.geode_robot.0).0;
    let max_rc = blueprint.obsidian_robot.1.0;
    let max_rob = blueprint.geode_robot.1.0;

    if memo.len() >= 100000000 {
        return Geode(i32::MAX);
    }

    if o > max_ore {
        o = max_ore;
    }

    if steps == 0 {
        return g;
    }

    let mut alternatives = Vec::new();
    let mut can_useful_robot = false;

    if o >= blueprint.geode_robot.0 && ob >= blueprint.geode_robot.1 {
        alternatives.push((steps-1, ro, rc, rob, rg+1, o-blueprint.geode_robot.0+Ore(ro), c+Clay(rc), ob-blueprint.geode_robot.1+Obsidian(rob), g+Geode(rg)))
    }
    else {
        if o >= blueprint.ore_robot && ro < max_ro {
            alternatives.push((steps-1, ro+1, rc, rob, rg, o-blueprint.ore_robot+Ore(ro), c+Clay(rc), ob+Obsidian(rob), g+Geode(rg)))
        }
        if o >= blueprint.clay_robot && rc < max_rc {
            alternatives.push((steps-1, ro, rc+1, rob, rg, o-blueprint.clay_robot+Ore(ro), c+Clay(rc), ob+Obsidian(rob), g+Geode(rg)))
        }
        if o >= blueprint.obsidian_robot.0 && c >= blueprint.obsidian_robot.1 && rob < max_rob {
            can_useful_robot = true;
            alternatives.push((steps-1, ro, rc, rob+1, rg, o-blueprint.obsidian_robot.0+Ore(ro), c-blueprint.obsidian_robot.1+Clay(rc), ob+Obsidian(rob), g+Geode(rg)))
        }
        if !can_useful_robot {
            alternatives.push((steps-1, ro, rc, rob, rg, o+Ore(ro), c+Clay(rc), ob+Obsidian(rob), g+Geode(rg)));
        }
    }

    alternatives.into_iter().map(|alternative| {
        if memo.contains_key(&alternative) {
            statistics.0 += 1;
            memo[&alternative]
        }
        else {
            statistics.1 += 1;
            let res = solve_rec(alternative, blueprint, memo, statistics);
            memo.insert(alternative, res);
            res
        }
    })
    .max()
    .unwrap()
}

pub fn complex()
{
    let blueprints = read_blueprints();

    let mut ans = 1;
    let mut statistics = (0, 0);
    for blueprint in blueprints.iter().take(3) {
        let mut memo = HashMap::new();
        let res = solve_rec((32, 1, 0, 0, 0, Ore(0), Clay(0), Obsidian(0), Geode(0)), blueprint, &mut memo, &mut statistics);
        println!("{res:?}");
        println!("Statistics: {statistics:?}");
        println!("memo.len(): {}", memo.len());
        ans *= res.0;
    }
    println!("{ans}");
}
