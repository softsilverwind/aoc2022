use std::io::{self, BufRead, BufReader};

fn num_to_snafu(mut num: i64) -> String
{
    let mut ans: Vec<u8> = Vec::new();
    let mut carry = 0;

    while (num + carry) > 0 {
        let byte;
        (byte, carry) = match (num + carry) % 5 {
            0 => (b'0', carry),
            1 => (b'1', 0),
            2 => (b'2', 0),
            3 => (b'=', 1),
            4 => (b'-', 1),
            _ => panic!()
        };

        ans.push(byte);
        num /= 5;
    }

    ans.reverse();

    String::from_utf8(ans).unwrap()
}

fn snafu_to_num(snafu: &str) -> i64
{
    snafu.bytes()
        .fold(0, |acc, elem| {
            let num = match elem {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => panic!()
            };

            acc * 5 + num
        })
}

fn read_input() -> Vec<String>
{
    BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .collect()
}

pub fn simple()
{
    let mason = read_input();
    let sum: i64 = mason.into_iter().map(|snafu| snafu_to_num(&snafu)).sum();
    println!("{}", num_to_snafu(sum));
}

pub fn complex()
{
}
