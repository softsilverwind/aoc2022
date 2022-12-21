use std::{
    io::{self, BufRead, BufReader},
    collections::HashMap
};

use regex::Regex;

#[derive(Debug)]
enum Op
{
    Add, Sub, Div, Mul
}

#[derive(Debug)]
enum Monkey
{
    Int(i64),
    Binop(Op, String, String)
}

fn eval(name: &str, monkeys: &HashMap<String, Monkey>) -> i64
{
    let ret = match &monkeys[name] {
        Monkey::Int(x) => *x,
        Monkey::Binop(op, left, right) => {
            let leftval = eval(left, monkeys);
            let rightval = eval(right, monkeys);

            match op {
                Op::Add => leftval + rightval,
                Op::Sub => leftval - rightval,
                Op::Div => leftval / rightval,
                Op::Mul => leftval * rightval,
            }
        }
    };

    ret
}

fn read_input() -> HashMap<String, Monkey>
{
    let regex_lit = Regex::new(r"([a-z]{4}): (\d+)").unwrap();
    let regex_op = Regex::new(r"([a-z]{4}): ([a-z]{4}) ([\+\-\*/]) ([a-z]{4})").unwrap();

    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            if let Some(cap) = regex_lit.captures(&line) {
                let name = cap[1].to_string();
                let number = cap[2].parse().unwrap();

                (name, Monkey::Int(number))
            }
            else if let Some(cap) = regex_op.captures(&line) {
                let name = cap[1].to_string();
                let left = cap[2].to_string();
                let right = cap[4].to_string();

                let op = match &cap[3] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "/" => Op::Div,
                    "*" => Op::Mul,
                    _ => panic!("monkaS")
                };

                (name, Monkey::Binop(op, left, right))
            }
            else {
                panic!("monkaS")
            }
        })
        .collect()
}

pub fn simple()
{
    let monkeys = read_input();
    println!("{}", eval("root", &monkeys));
}

pub fn complex()
{
    let mut monkeys = read_input();

    let Monkey::Binop(op, _, _) = monkeys.get_mut("root").unwrap() else { panic!() };
    *op = Op::Sub;

    // This could (almost) be an one-liner if we had binary search on ranges :)

    // E.g., in Ruby:
    // (1..).lazy.map{|x| 2**(x-1)..2**x}.find{|x| pred(x.end)}.bsearch{|x| pred(x)}
    let mut end: i64 = 2;
    *monkeys.get_mut("humn").unwrap() = Monkey::Int(end);
    let mut prev = eval("root", &monkeys);
    let mut curr = prev;

    while curr.signum() == prev.signum() {
        end *= 2;

        prev = curr;
        *monkeys.get_mut("humn").unwrap() = Monkey::Int(end);
        curr = eval("root", &monkeys);
    }

    let mut start = end / 2;
    let mut start_e = prev;
    let mut end_e = curr;
    let mut middle = (end + start) / 2;
    *monkeys.get_mut("humn").unwrap() = Monkey::Int(middle);
    let mut middle_e = eval("root", &monkeys);

    while middle_e.signum() != 0 {
        if middle_e.signum() == end_e.signum() {
            end = middle;
            end_e = middle_e;
        }
        else if middle_e.signum() == start_e.signum() {
            start = middle;
            start_e = middle_e;
        }

        middle = (end + start) / 2;
        *monkeys.get_mut("humn").unwrap() = Monkey::Int(middle);
        middle_e = eval("root", &monkeys);
    }

    println!("{middle}");
}
