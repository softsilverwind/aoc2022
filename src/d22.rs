use std::{
    io::{self, BufRead, BufReader},
    collections::HashSet
};

use itertools::Itertools;

struct Field
{
    vertical_bounds: Vec<(usize, usize)>,
    horizontal_bounds: Vec<(usize, usize)>,
    walls: HashSet<(usize, usize)>
}

#[derive(Debug)]
enum Instruction
{
    Move(i32), RotateRight, RotateLeft
}

#[derive(Clone, Copy, Debug)]
enum Direction
{
    Up, Left, Down, Right
}

impl Direction
{
    fn rotate_right(self) -> Direction
    {
        use Direction::*;
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }

    fn rotate_left(self) -> Direction
    {
        use Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn number(&self) -> usize
    {
        use Direction::*;
        match self {
            Up => 3,
            Left => 2,
            Down => 1,
            Right => 0,
        }
    }
}

fn read_input() -> (Field, Vec<Instruction>)
{
    let mut lines = BufReader::new(io::stdin()).lines().map(|read_line| read_line.unwrap());
    let mut input: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        input.push(line);
    }

    let ymax = input.len();
    let xmax = input.iter().map(|line| line.len()).max().unwrap();

    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut horizontal_bounds = vec![(0, 0); ymax + 1];
    let mut vertical_bounds = vec![(0, 0); xmax + 1];

    let bytes: Vec<Vec<u8>> = input.iter().map(|line| line.bytes().collect()).collect();

    for y in 0..ymax {
        for x in 0..xmax {
            if bytes[y].get(x).copied() == Some(b'#') {
                walls.insert((x + 1, y + 1));
            }
        }
    }

    for y in 0..ymax {
        let mut first_space = true;
        let hb = &mut horizontal_bounds[y + 1]; 
        for x in 0..xmax {
            match bytes[y].get(x).copied() {
                Some(b' ') | None => {
                    if first_space {
                        hb.0 = x + 1;
                    }
                    else if hb.1 == 0 {
                        hb.1 = x + 1;
                        break;
                    }
                },
                _ => {
                    first_space = false;
                }
            }
        }
        if hb.1 == 0 {
            hb.1 = xmax + 1;
        }
    }

    for x in 0..xmax {
        let mut first_space = true;
        let vb = &mut vertical_bounds[x + 1]; 
        for y in 0..ymax {
            match bytes[y].get(x).copied() {
                Some(b' ') | None => {
                    if first_space {
                        vb.0 = y + 1;
                    }
                    else if vb.1 == 0 {
                        vb.1 = y + 1;
                        break;
                    }
                },
                _ => {
                    first_space = false;
                }
            }
        }
        if vb.1 == 0 {
            vb.1 = ymax + 1;
        }
    }

    let instructions: Vec<Instruction> = lines.next().unwrap()
        .chars()
        .group_by(|&x| x == 'L' || x == 'R')
        .into_iter()
        .map(|(_, text_iter)| {
            let text: String = text_iter.collect();

            match text.as_ref() {
                "L" => Instruction::RotateLeft,
                "R" => Instruction::RotateRight,
                _ => Instruction::Move(text.parse().unwrap())
            }
        })
        .collect();

    let field = Field {
        vertical_bounds,
        horizontal_bounds,
        walls
    };
    (field, instructions)

}

pub fn simple()
{
    let (field, instructions) = read_input();

    let (mut x, mut y) = (field.horizontal_bounds[1].0 + 1, 1);
    let mut direction = Direction::Right;

    for instruction in instructions {
        match instruction {
            Instruction::Move(steps) => for _ in 0..steps {
                let (mut xnext, mut ynext) = match direction {
                    Direction::Up => (x, y-1),
                    Direction::Left => (x-1, y),
                    Direction::Down => (x, y+1),
                    Direction::Right => (x+1, y)
                };

                match direction {
                    Direction::Up => if ynext == field.vertical_bounds[x].0 { ynext = field.vertical_bounds[x].1 - 1 },
                    Direction::Left => if xnext == field.horizontal_bounds[y].0 { xnext = field.horizontal_bounds[y].1 - 1 },
                    Direction::Down => if ynext == field.vertical_bounds[x].1 { ynext = field.vertical_bounds[x].0 + 1 },
                    Direction::Right => if xnext == field.horizontal_bounds[y].1 { xnext = field.horizontal_bounds[y].0 + 1 },
                }

                if !field.walls.contains(&(xnext, ynext)) {
                    (x, y) = (xnext, ynext);
                }
            },
            Instruction::RotateRight => direction = direction.rotate_right(),
            Instruction::RotateLeft => direction = direction.rotate_left(),
        }
    }

    println!("{}", y * 1000 + x * 4 + direction.number());
}

pub fn complex()
{
}
