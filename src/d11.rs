use std::{
    io::{self, BufRead, BufReader},
    collections::{HashMap, VecDeque}
};
use regex::Regex;

#[derive(Debug, Default)]
enum Operation
{
    Add(i32),
    Mul(i32),
    #[default]
    Sq
}

#[derive(Debug, Default)]
struct Monke
{
    pub items: VecDeque<i32>,
    pub operation: Operation,
    pub divisible_by: i32,
    pub next_true: usize,
    pub next_false: usize,
    pub inspections: i32
}

fn read_monkas() -> Vec<Monke>
{
    let mut monkas: Vec<Monke> = Vec::new();
    let items_regex = Regex::new(r"Starting items: ([\d ,]+)").unwrap();
    let operation_regex = Regex::new(r"Operation: new = old ([\+\*]) (old|\d+)").unwrap();
    let test_regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_regex = Regex::new(r"If true: throw to monkey (\d)").unwrap();
    let false_regex = Regex::new(r"If false: throw to monkey (\d)").unwrap();
    
    for line in BufReader::new(io::stdin()).lines().map(|read_line| read_line.unwrap()) {
        if line.contains("Monkey") {
            monkas.push(Monke::default())
        }
        else if let Some(cap) = items_regex.captures(&line) {
            let numbers = cap[1].split(", ").map(|x| x.parse::<i32>().unwrap());
            monkas.last_mut().unwrap().items.extend(numbers);
        }
        else if let Some(cap) = operation_regex.captures(&line) {
            monkas.last_mut().unwrap().operation = match (&cap[1], &cap[2]) {
                ("+", number) => Operation::Add(number.parse().unwrap()),
                ("*", "old") => Operation::Sq,
                ("*", number) => Operation::Mul(number.parse().unwrap()),
                _ => panic!()
            }
        }
        else if let Some(cap) = test_regex.captures(&line) {
            monkas.last_mut().unwrap().divisible_by = cap[1].parse().unwrap();
        }
        else if let Some(cap) = true_regex.captures(&line) {
            monkas.last_mut().unwrap().next_true = cap[1].parse().unwrap();
        }
        else if let Some(cap) = false_regex.captures(&line) {
            monkas.last_mut().unwrap().next_false = cap[1].parse().unwrap();
        }
    }

    monkas
}

pub fn simple()
{
    let mut monkas = read_monkas();

    for _ in 0..20 {
        for i in 0..monkas.len() {
            while let Some(mut worry) = monkas[i].items.pop_front() {
                monkas[i].inspections += 1;

                match monkas[i].operation {
                    Operation::Add(x) => worry += x,
                    Operation::Mul(x) => worry *= x,
                    Operation::Sq => worry *= worry
                }

                worry /= 3;

                let next = if worry % monkas[i].divisible_by == 0 { monkas[i].next_true } else { monkas[i].next_false };
                monkas[next].items.push_back(worry);
            }
        }
    }

    monkas.sort_by_key(|monka| -monka.inspections);

    println!("{:?}", monkas[0].inspections * monkas[1].inspections);
}

type Item = HashMap<i32, i32>;

#[derive(Debug, Default)]
struct Monke2
{
    pub items: VecDeque<Item>,
    pub operation: Operation,
    pub divisible_by: i32,
    pub next_true: usize,
    pub next_false: usize,
    pub inspections: i64
}

impl Monke2
{
    fn from_monke(
        Monke { items, operation, divisible_by, next_true, next_false, inspections } : Monke,
        divisors: &[i32]
    ) -> Monke2
    {
        let items = items.into_iter()
            .map(|value| {
                let mut item = Item::new();

                for divisor in divisors.iter().copied() {
                    item.insert(divisor, value % divisor);
                }

                item
            })
            .collect();

        Monke2 {
            items,
            operation,
            divisible_by,
            next_true,
            next_false,
            inspections: inspections as i64
        }
    }
}

pub fn complex()
{
    let old_monkas = read_monkas();

    let divisors: Vec<i32> = old_monkas.iter().map(|monka| monka.divisible_by).collect();
    
    let mut monkas: Vec<Monke2> = old_monkas.into_iter().map(|monke| Monke2::from_monke(monke, &divisors)).collect();

    for _ in 0..10000 {
        for i in 0..monkas.len() {
            while let Some(mut item) = monkas[i].items.pop_front() {
                monkas[i].inspections += 1;

                match monkas[i].operation {
                    Operation::Add(x) => {
                        for (key, val) in item.iter_mut() {
                            *val += x;
                            *val %= key;
                        }
                    }
                    Operation::Mul(x) => {
                        for (key, val) in item.iter_mut() {
                            *val *= x;
                            *val %= key;
                        }
                    }
                    Operation::Sq => {
                        for (key, val) in item.iter_mut() {
                            *val *= *val;
                            *val %= key;
                        }
                    }
                }

                let next = if item[&monkas[i].divisible_by] == 0 { monkas[i].next_true } else { monkas[i].next_false };
                monkas[next].items.push_back(item);
            }
        }
    }

    monkas.sort_by_key(|monka| -monka.inspections);

    println!("{:?}", monkas[0].inspections * monkas[1].inspections);
}
