use std::io::{self, BufRead, BufReader};

pub fn simple()
{
    let grid: Vec<Vec<i8>> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            line.bytes().filter_map(|byte|
                if byte >= b'0' && byte <= b'9' {
                    Some((byte - b'0') as i8)
                }
                else {
                    None
                }
            )
            .collect()
        })
        .collect();

    let mut visibility: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        let mut max_height = -1;
        // Left-to-right
        for j in 0..grid[i].len() {
            if grid[i][j] > max_height {
                visibility[i][j] = true;
                max_height = grid[i][j];
            }
        }

        // Right-to-left
        max_height = -1;
        for j in (0..grid[i].len()).rev() {
            if grid[i][j] > max_height {
                visibility[i][j] = true;
                max_height = grid[i][j];
            }
        }
    }

    for j in 0..grid[0].len() {
        // Top-down
        let mut max_height = -1;
        for i in 0..grid.len() {
            if grid[i][j] > max_height {
                visibility[i][j] = true;
                max_height = grid[i][j];
            }
        }

        // Bottom-up
        max_height = -1;
        for i in (0..grid.len()).rev() {
            if grid[i][j] > max_height {
                visibility[i][j] = true;
                max_height = grid[i][j];
            }
        }
    }

    println!("{}", visibility.iter().map(|line| line.iter().map(|&elem| if elem { 1 } else { 0 }).sum::<i32>()).sum::<i32>());
}

pub fn complex()
{
    let grid: Vec<Vec<i8>> = BufReader::new(io::stdin()).lines()
        .map(|read_line| read_line.unwrap())
        .map(|line| {
            line.bytes().filter_map(|byte|
                if byte >= b'0' && byte <= b'9' {
                    Some((byte - b'0') as i8)
                }
                else {
                    None
                }
            )
            .collect()
        })
        .collect();

    let max_i = grid.len();
    let max_j = grid[0].len();

    let mut max_scenic_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let (mut ssu, mut ssd, mut ssl, mut ssr) = (0, 0, 0, 0);
            let height = grid[i][j];
            for k in (0..i).rev() { ssu += 1; if grid[k][j] >= height { break } } // Up
            for k in (i+1)..max_i { ssd += 1; if grid[k][j] >= height { break } } // Down
            for k in (0..j).rev() { ssl += 1; if grid[i][k] >= height { break } } // Left
            for k in (j+1)..max_j { ssr += 1; if grid[i][k] >= height { break } } // Right

            let scenic_score = ssu * ssd * ssl * ssr;
            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }

    println!("{}", max_scenic_score);
}
