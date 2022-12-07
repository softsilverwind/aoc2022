use std::io::{self, BufRead, BufReader};

pub fn simple()
{
    let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            match line.as_ref() {
                "A X" => 4,
                "B X" => 1,
                "C X" => 7,
                "A Y" => 8,
                "B Y" => 5,
                "C Y" => 2,
                "A Z" => 3,
                "B Z" => 9,
                "C Z" => 6,
                _ => panic!()
            }
        })
        .sum::<i32>();

    println!("{}", ans);
}

pub fn complex()
{
    let ans = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            match line.as_ref() {
                "A X" => 3,
                "B X" => 1,
                "C X" => 2,
                "A Y" => 4,
                "B Y" => 5,
                "C Y" => 6,
                "A Z" => 8,
                "B Z" => 9,
                "C Z" => 7,
                _ => panic!()
            }
        })
        .sum::<i32>();

    println!("{}", ans);

}
