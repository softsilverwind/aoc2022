use std::{
    io::{self, BufRead, BufReader},
    collections::HashSet
};

enum Command
{
    Noop,
    Add(i32),
}

pub fn simple()
{
    let commands = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .flat_map(|line| {
            let cmd_str: Vec<&str> = line.split(' ').collect();

            match cmd_str[0] {
                "noop" => vec![Command::Noop],
                "addx" => vec![Command::Noop, Command::Add(cmd_str[1].parse().unwrap())],
                _ => panic!()
            }
        });

    let breakpoints: HashSet<i32> = vec![20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut clock = 0;
    let mut register = 1;
    let mut result = 0;

    for cmd in commands {
        clock += 1;
        if breakpoints.contains(&clock) {
            result += clock * register;
        }
        if clock >= 220 {
            break;
        }

        match cmd {
            Command::Noop => (),
            Command::Add(x) => register += x
        }
    }

    println!("{}", result);
}

pub fn complex()
{
    let commands = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .flat_map(|line| {
            let cmd_str: Vec<&str> = line.split(' ').collect();

            match cmd_str[0] {
                "noop" => vec![Command::Noop],
                "addx" => vec![Command::Noop, Command::Add(cmd_str[1].parse().unwrap())],
                _ => panic!()
            }
        });

    let mut clock: i32 = 0;
    let mut register = 1;

    for cmd in commands {
        clock += 1;

        let crt_position = (clock - 1) % 40;

        if (crt_position - register).abs() <= 1 {
            print!("#");
        }
        else {
            print!(".");
        }
        if crt_position == 39 {
            println!();
        }

        match cmd {
            Command::Noop => (),
            Command::Add(x) => register += x
        }
    }
}
