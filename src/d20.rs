use std::io::{self, BufRead, BufReader};

use nicole::{VecList, ForwardIndex, IndexExt, BackwardIndex};

fn solve(decrypt_key: i64, mix_count: usize)
{
    let mut mason = VecList::new();
    let mut indices = Vec::new();
    
    for line in BufReader::new(io::stdin()).lines().map(|read_line| read_line.unwrap()) {
        mason.push_back(line.parse::<i64>().unwrap() * decrypt_key);
    }

    let mut index = mason.begin();

    while mason.valid(&index) {
        indices.push(index);
        mason.increment(&mut index);
    }

    let mut zero_index = None;

    for _ in 0..mix_count {
        for pi in 0..indices.len() {
            let start_pos = indices[pi];
            let number = mason[start_pos];

            if number == 0 {
                zero_index = Some(start_pos);
                continue;
            }

            let mut end_pos = start_pos;
            let mut count = number;

            while count >= mason.len() as i64 {
                count %= mason.len() as i64 - 1;
            }

            while count <= -(mason.len() as i64) {
                count %= mason.len() as i64 - 1;
            }

            if count > 0 {
                while count >= 0 {
                    count -= 1;
                    mason.increment(&mut end_pos);
                    if !mason.valid(&end_pos) {
                        end_pos = mason.begin();
                    }
                }
            }
            else { 
                while count < 0 {
                    count += 1;
                    mason.decrement(&mut end_pos);
                    if !mason.valid(&end_pos) {
                        end_pos = mason.end();
                    }
                }
            }

            if start_pos != end_pos {
                mason.remove(start_pos);
                mason.insert(end_pos, number);
                indices[pi] = mason.prev(end_pos);
            }
        }
    }

    index = zero_index.unwrap();

    let mut count = 1;
    let mut ans = 0;

    while count <= 3000 {
        mason.increment(&mut index);
        if !mason.valid(&index) {
            index = mason.begin();
        }
        if count == 1000 || count == 2000 || count == 3000 {
            ans += mason[index];
        }
        count += 1;
    }

    println!("{ans}");
}

pub fn simple()
{
    solve(1, 1);
}

pub fn complex()
{
    solve(811589153, 10);
}
