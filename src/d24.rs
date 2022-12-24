use std::{
    io::{self, BufRead, BufReader},
    collections::{HashMap, HashSet, BTreeMap}
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction
{
    Up, Left, Down, Right
}

impl Direction
{
    fn new(byte: u8) -> Self
    {
        match byte {
            b'^' => Direction::Up,
            b'<' => Direction::Left,
            b'v' => Direction::Down,
            b'>' => Direction::Right,
            _ => panic!()
        }
    }

    fn all() -> [Self; 4]
    {
        use Direction::*;
        [Up, Left, Down, Right]
    }

    fn advance(&self, (i, j): (i32, i32)) -> (i32, i32)
    {
        use Direction::*;

        match self {
            Up => (i-1, j),
            Left => (i, j-1),
            Down => (i+1, j),
            Right => (i, j+1)
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Blizzard
{
    pos: (i32, i32),
    direction: Direction
}

#[derive(Debug)]
struct Field
{
    start: (i32, i32),
    end: (i32, i32),
    min: (i32, i32),
    max: (i32, i32),
    blizzards: Vec<Blizzard>
}

fn read_input() -> Field
{
    let input: Vec<Vec<u8>> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| line.bytes().collect())
        .collect();

    let mut blizzards = Vec::new();
    let max = (input.len() as i32 - 2, input[0].len() as i32 - 2);

    for i in 1..=max.0 {
        for j in 1..=max.1 {
            if input[i as usize][j as usize] != b'.' {
                blizzards.push(Blizzard { pos: (i, j), direction: Direction::new(input[i as usize][j as usize]) });
            }
        }
    }

    let start = (0, 1);
    let min = (1, 1);
    let end = (max.0 + 1, max.1);

    Field {
        start, end, min, max, blizzards
    }
}

fn simulate_blizzards(field: &Field) -> HashMap<i32, HashSet<(i32, i32)>>
{
    let mut visited: HashSet<Vec<Blizzard>> = HashSet::new();
    let mut blizzards = field.blizzards.clone();
    let mut obstacles = HashMap::new();
    let mut i = 0;

    loop {
        if !visited.insert(blizzards.clone()) {
            break;
        }

        obstacles.insert(i, blizzards.iter().map(|blizzard| blizzard.pos.clone()).collect());

        for blizzard in blizzards.iter_mut() {
            let mut next = blizzard.direction.advance(blizzard.pos);

            if next.0 < field.min.0 {
                next.0 = field.max.0;
            }
            else if next.0 > field.max.0 {
                next.0 = field.min.0;
            }
            else if next.1 < field.min.1 {
                next.1 = field.max.1;
            }
            else if next.1 > field.max.1 {
                next.1 = field.min.1;
            }

            blizzard.pos = next;
        }

        i += 1;
    }

    obstacles
}

struct State
{
    pos: (i32, i32),
    time: i32
}

fn dist(from: (i32, i32), to: (i32, i32)) -> i32
{
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

#[allow(unused)]
fn debug(field: &Field, pos: (i32, i32), obstacles: &HashSet<(i32, i32)>)
{
    println!("{}", std::iter::once('#').cycle().take(field.max.1 as usize + 2).join(""));
    for i in field.min.0..=field.max.0 {
        print!("#");
        for j in field.min.1..=field.max.1 {
            if obstacles.contains(&(i, j)) && pos == (i, j) {
                print!("D");
            }
            else if obstacles.contains(&(i, j)) {
                print!("*");
            }
            else if pos == (i, j) {
                print!("E");
            }
            else {
                print!(".");
            }
        }
        println!("#");
    }
    println!("{}", std::iter::once('#').cycle().take(field.max.1 as usize + 2).join(""));
}

fn solve(field: &Field, obstacles: &HashMap<i32, HashSet<(i32, i32)>>, time: i32) -> i32
{
    let mut front: BTreeMap<i32, Vec<State>> = BTreeMap::new();

    front.insert(dist(field.start, field.end), vec![State { pos: field.start, time }]);

    // let mut max_heuristic = 0;

    let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();

    while let Some((heuristic, mut state_vec)) = front.pop_first() {
        // if heuristic > max_heuristic {
        //    println!("{heuristic}: {}", front.values().map(|x| x.len()).sum::<usize>());
        //    max_heuristic = heuristic;
        // }
        let Some(current_state) = state_vec.pop() else { continue; };
        if !state_vec.is_empty() {
            front.insert(heuristic, state_vec);
        }

        let periodic_state = (current_state.pos, current_state.time % obstacles.len() as i32);

        if !visited.insert(periodic_state) {
            continue;
        }

        let next_moves = Direction::all().into_iter().map(|dir| dir.advance(current_state.pos));
        let next_stay = std::iter::once(current_state.pos);

        for next in next_moves.chain(next_stay) {
            if next == field.end {
                return current_state.time + 1;
            }

            if next != field.start && (next.0 < field.min.0 || next.1 < field.min.1 || next.0 > field.max.0 || next.1 > field.max.1) {
                continue;
            }

            let time = current_state.time + 1;
            let periodic_time = time % obstacles.len() as i32;

            if obstacles[&periodic_time].contains(&next) {
                continue;
            }

            front.entry(time + dist(next, field.end)).or_insert_with(|| Vec::new()).push(State { pos: next, time });
        }
    }

    return -1;
}

pub fn simple()
{
    let field = read_input();
    let obstacles = simulate_blizzards(&field);
    let ans = solve(&field, &obstacles, 0);
    println!("{ans}");
}

pub fn complex()
{
    let mut field = read_input();
    let obstacles = simulate_blizzards(&field);
    let time1 = solve(&field, &obstacles, 0);
    std::mem::swap(&mut field.start, &mut field.end);
    let time2 = solve(&field, &obstacles, time1);
    std::mem::swap(&mut field.start, &mut field.end);
    let time3 = solve(&field, &obstacles, time2);

    println!("{time3}");
}
