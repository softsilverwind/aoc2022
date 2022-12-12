use std::{
    io::{self, BufRead, BufReader},
    collections::{HashSet, VecDeque}
};

struct Field
{
    elevations: Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize)
}

fn read_field() -> Field
{
    let mut start = (0, 0);
    let mut end = (0, 0);
    
    let elevations: Vec<Vec<i32>> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .enumerate()
        .map(|(i, line)|
            line.bytes().enumerate().filter_map(|(j, byte)| {
                if byte >= b'a' && byte <= b'z' {
                    Some(byte - b'a')
                }
                else if byte == b'S' {
                    start = (i, j);
                    Some(0)
                }
                else if byte == b'E' {
                    end = (i, j);
                    Some(b'z' - b'a')
                }
                else {
                    None
                }
            })
            .map(|x| x as i32)
            .collect()
        )
        .collect();

    Field {
        elevations, start, end
    }
}

pub fn simple()
{
    let field = read_field();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut front: VecDeque<(i32, (usize, usize))> = VecDeque::new();

    front.push_back((0, field.start));

    while let Some((steps, (i, j))) = front.pop_front() {
        if visited.contains(&(i, j)) {
            continue;
        }

        visited.insert((i, j));

        if (i, j) == field.end {
            println!("{}", steps);
            break;
        }

        for &(ni, nj) in &[(i+1, j), (i.wrapping_sub(1), j), (i, j+1), (i, j.wrapping_sub(1))] {
            if ni < field.elevations.len() && nj < field.elevations[0].len()
                && field.elevations[ni][nj] - field.elevations[i][j] <= 1 {
                front.push_back((steps + 1, (ni, nj)));
            }
        }
    }
}

pub fn complex()
{
    let field = read_field();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut front: VecDeque<(i32, (usize, usize))> = VecDeque::new();

    for i in 0..field.elevations.len() {
        for j in 0..field.elevations[0].len() {
            if field.elevations[i][j] == 0 {
                front.push_back((0, (i, j)));
            }
        }
    }

    while let Some((steps, (i, j))) = front.pop_front() {
        if visited.contains(&(i, j)) {
            continue;
        }

        visited.insert((i, j));

        if (i, j) == field.end {
            println!("{}", steps);
            break;
        }

        for &(ni, nj) in &[(i+1, j), (i.wrapping_sub(1), j), (i, j+1), (i, j.wrapping_sub(1))] {
            if ni < field.elevations.len() && nj < field.elevations[0].len()
                && field.elevations[ni][nj] - field.elevations[i][j] <= 1 {
                front.push_back((steps + 1, (ni, nj)));
            }
        }
    }
}
