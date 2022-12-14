use std::{
    io::{self, BufRead, BufReader},
    collections::HashSet
};

use itertools::Itertools;

fn read_field() -> (HashSet<(i32, i32)>, i32)
{
    let mut field: HashSet<(i32, i32)> = HashSet::new();
    let mut lowest = 0;

    for line in BufReader::new(io::stdin()).lines().map(|read_line| read_line.unwrap()) {
        let points: Vec<(i32, i32)> = line.split(" -> ")
            .map(|s| {
                let (s1, s2) = s.split(',').next_tuple().unwrap();
                (s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap())
            })
            .collect();

        for ((x1, y1), (x2, y2)) in points.into_iter().tuple_windows() {
            let (dx, dy) = ((x2 - x1).signum(), (y2 - y1).signum());

            lowest = lowest.max(y1).max(y2);

            let (mut x, mut y) = (x1, y1);

            while (x, y) != (x2, y2) {
                field.insert((x, y));

                x += dx;
                y += dy;
            }

            field.insert((x2, y2));
        }
    }

    (field, lowest)
}

pub fn simple()
{
    let (mut field, lowest) = read_field();

    let mut grains = 0;
    loop {
        grains += 1;
        let (mut x, mut y) = (500, 0);

        loop {
            if y >= lowest {
                break;
            }
            else if !field.contains(&(x, y+1)) {
                y += 1;
            }
            else if !field.contains(&(x-1, y+1)) {
                x -= 1;
                y += 1;
            }
            else if !field.contains(&(x+1, y+1)) {
                x += 1;
                y += 1;
            }
            else {
                field.insert((x, y));
                break;
            }
        }

        if y >= lowest {
            break;
        }
    }
    println!("{}", grains - 1);
}

pub fn complex()
{
    let (mut field, lowest) = read_field();

    let mut grains = 0;
    loop {
        grains += 1;
        let (mut x, mut y) = (500, 0);

        loop {
            if y == lowest + 1 {
                field.insert((x, y));
                break;
            }
            else if !field.contains(&(x, y+1)) {
                y += 1;
            }
            else if !field.contains(&(x-1, y+1)) {
                x -= 1;
                y += 1;
            }
            else if !field.contains(&(x+1, y+1)) {
                x += 1;
                y += 1;
            }
            else {
                field.insert((x, y));
                break;
            }
        }

        if field.contains(&(500, 0)) {
            break;
        }
    }
    println!("{}", grains);
}
