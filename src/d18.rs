use std::{
    io::{self, BufRead, BufReader},
    collections::BTreeSet
};
use itertools::Itertools;

type Coords = (i32, i32, i32);

macro_rules! neighbours {
    ($x: ident, $y: ident, $z: ident) => {
        [
            ($x+1, $y  , $z  ),
            ($x-1, $y  , $z  ),
            ($x  , $y+1, $z  ),
            ($x  , $y-1, $z  ),
            ($x  , $y  , $z+1),
            ($x  , $y  , $z-1)
        ]
    }
}

fn read_input() -> Vec<Coords>
{
    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| line.split(',').map(|coord| coord.parse::<i32>().unwrap()).next_tuple().unwrap())
        .collect()
}

fn solve(set: &BTreeSet<Coords>) -> usize
{
    set.iter().copied().map(|(x, y, z)| {
        let adj: BTreeSet<Coords> = neighbours!(x, y, z).into_iter().collect();

        adj.difference(&set).count()
    })
    .sum()
}

pub fn simple()
{
    let input = read_input();
    let set: BTreeSet<_> = input.into_iter().collect();
    let ans = solve(&set);
    println!("{ans}");
}

fn out_of_bounds((x, y, z): (i32, i32, i32) , (xmax, ymax, zmax): (i32, i32, i32)) -> bool
{
    x > xmax || y > ymax || z > zmax
        || x < 0 || y < 0 || z < 0
}

pub fn complex()
{
    let input = read_input();

    let max @ (_, _, _) = input.iter().copied().fold((0, 0, 0), |(xmax, ymax, zmax), (x, y, z)| (xmax.max(x), ymax.max(y), zmax.max(z)));

    let set: BTreeSet<_> = input.into_iter().collect();

    let (mut sum, pockets) = set.iter().copied().fold((0, BTreeSet::new()), |(sum, mut pockets), (x, y, z)| {
        let adj: BTreeSet<Coords> = neighbours!(x, y, z).into_iter().collect();

        let mut difference: BTreeSet<_> = adj.difference(&set).copied().collect();
        let count = difference.len();
        pockets.append(&mut difference);

        (sum + count, pockets)
    });

    let mut global_visited: BTreeSet<Coords> = BTreeSet::new();

    for init in pockets {
        if global_visited.contains(&init) { continue; }

        let mut front = vec![init];
        let mut visited: BTreeSet<Coords> = BTreeSet::new();
        let mut outside = false;

        while let Some(point @ (x, y, z)) = front.pop() {
            if visited.contains(&point) || set.contains(&point) {
                continue;
            }

            if out_of_bounds(point, max) {
                outside = true;
                break;
            }

            visited.insert(point);
            front.extend(neighbours!(x, y, z).into_iter());
        }

        if !outside {
            sum -= solve(&visited);
        }

        global_visited.append(&mut visited);
    }

    println!("{sum}");
}
