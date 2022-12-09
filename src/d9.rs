use std::{
    io::{self, BufRead, BufReader},
    collections::HashSet
};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Dir
{
    Up, Down, Left, Right
}

pub fn simple()
{
    let commands: Vec<Dir> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .flat_map(|line| {
            let (d, n) = line.split(' ').next_tuple().unwrap();
            let dir = match d {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!()
            };
            let num = n.parse::<i32>().unwrap();

            (0..num).map(move |_| dir)
        })
        .collect();

    let mut head: (i32, i32) = (0, 0);
    let mut tail = head;
    let mut set = HashSet::new();
    set.insert(tail);

    for command in commands {
        match command {
            Dir::Up => head.1 += 1,
            Dir::Down => head.1 -= 1,
            Dir::Left => head.0 -= 1,
            Dir::Right => head.0 += 1
        }

        if (head.0 - tail.0).abs() >= 2 || (head.1 - tail.1).abs() >= 2 {
            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();
            set.insert(tail);
        }
    }

    println!("{}", set.len());
}

pub fn complex()
{
    let commands: Vec<Dir> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .flat_map(|line| {
            let (d, n) = line.split(' ').next_tuple().unwrap();
            let dir = match d {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!()
            };
            let num = n.parse::<i32>().unwrap();

            (0..num).map(move |_| dir)
        })
        .collect();

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut set = HashSet::new();
    set.insert(rope[0]);

    for command in commands {
        match command {
            Dir::Up => rope[0].1 += 1,
            Dir::Down => rope[0].1 -= 1,
            Dir::Left => rope[0].0 -= 1,
            Dir::Right => rope[0].0 += 1
        }

        for i in 1..10 {
            if (rope[i-1].0 - rope[i].0).abs() >= 2 || (rope[i-1].1 - rope[i].1).abs() >= 2 {
                rope[i].0 += (rope[i-1].0 - rope[i].0).signum();
                rope[i].1 += (rope[i-1].1 - rope[i].1).signum();
            }
        }
        set.insert(rope[9]);
    }

    println!("{}", set.len());
}
