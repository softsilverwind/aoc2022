use std::{
    io::{self, BufRead, BufReader},
    collections::{HashMap, HashSet, VecDeque}
};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Direction
{
    North, South, West, East
}

impl Direction
{
    fn all() -> [Direction; 4]
    {
        use Direction::*;
        [North, South, West, East]
    }

    fn conflict(&self, (x, y): (i32, i32), elf_positions: &HashSet<(i32, i32)>) -> bool
    {
        use Direction::*;

        match self {
            North => [(x-1, y-1), (x, y-1), (x+1, y-1)].iter().any(|pos| elf_positions.contains(pos)),
            South => [(x-1, y+1), (x, y+1), (x+1, y+1)].iter().any(|pos| elf_positions.contains(pos)),
            West =>  [(x-1, y-1), (x-1, y), (x-1, y+1)].iter().any(|pos| elf_positions.contains(pos)),
            East =>  [(x+1, y-1), (x+1, y), (x+1, y+1)].iter().any(|pos| elf_positions.contains(pos)),
        }
    }

    fn advance(&self, (x, y): (i32, i32)) -> (i32, i32)
    {
        use Direction::*;

        match self {
            North => (x, y-1),
            South => (x, y+1),
            West => (x-1, y),
            East => (x+1, y)
        }
    }
}

struct Elf
{
    x: i32,
    y: i32,
    consideration: Option<Direction>
}

impl Elf
{
    fn new(x: i32, y: i32) -> Self
    {
        Elf { x, y, consideration: None }
    }
}

fn read_input() -> Vec<Elf>
{
    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .enumerate()
        .flat_map(|(y, line)|
            line.bytes().enumerate().filter_map(|(x, byte)|
                if byte == b'#' {
                    Some(Elf::new(x as i32, y as i32))
                }
                else {
                    None
                }
            )
            .collect::<Vec<_>>()
        )
        .collect()
}

fn solve(simple: bool)
{
    let mut elves = read_input();

    let mut priority: VecDeque<Direction> = Direction::all().into_iter().collect();

    for round in 1.. {
        // Consideration phase
        let elf_positions: HashSet<(i32, i32)> = elves.iter().map(|elf| (elf.x, elf.y)).collect();

        let mut considerations: HashMap<(i32, i32), i32> = HashMap::new();

        for elf in elves.iter_mut() {
            let pos = (elf.x, elf.y);

            if Direction::all().into_iter().map(|dir| dir.conflict(pos, &elf_positions)).all_equal() {
                continue;
            }

            let dir = *priority.iter().find(|dir| !dir.conflict(pos, &elf_positions)).unwrap();
            elf.consideration = Some(dir);
            *considerations.entry(dir.advance(pos)).or_insert(0) += 1;
        }

        // Move phase
        let mut moved = false;
        for elf in elves.iter_mut() {
            if let Some(next_dir) = elf.consideration {
                let next @ (nx, ny) = next_dir.advance((elf.x, elf.y));

                if considerations[&next] == 1 {
                    (elf.x, elf.y) = (nx, ny);
                    moved = true;
                }
            }
            elf.consideration = None;
        }

        let first = priority.pop_front().unwrap();
        priority.push_back(first);

        if simple && round == 10 {
            let elf_positions: HashSet<(i32, i32)> = elves.iter().map(|elf| (elf.x, elf.y)).collect();
            let ((xmin, ymin), (xmax, ymax)) = elves.iter().fold(((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)), |((xmin, ymin), (xmax, ymax)), elf|
                ((xmin.min(elf.x), ymin.min(elf.y)), (xmax.max(elf.x), ymax.max(elf.y)))
            );

            let mut count = 0;

            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    if !elf_positions.contains(&(x, y)) {
                        count += 1;
                    }
                }
            }

            println!("{count}");
            break;
        }
        else if !simple && !moved {
            println!("{round}");
            break;
        }
    }
}

pub fn simple()
{
    solve(true);
}

pub fn complex()
{
    solve(false);
}
