use std::prelude::v1::*;
use std::io::prelude::*;


fn solve( grid : Vec<Vec<u8>> ) {
    let n = grid.len();
    let step = [(0,0), (0, n -1 ), (n - 1, 0), (n - 1, n - 1)];
    let p: (usize, usize) = step.iter().cloned()
        .find(|&(i, j)| grid[i][j] == b'p').expect("no p");

    let mut m : (usize, usize) = (0,0);
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == b'm' {
                m = (i, j);
                break;
            }
        }
    }

    let v = m.0 as i32 - p.0 as i32;
    let h = m.1 as i32 - p.1 as i32;
    let step = if v < 0 { "DOWN" } else { "UP" };
    for _ in 0..v.abs() {
        println!("{}", step);
    }

    let step = if h < 0 { "RIGHT" } else { "LEFT" };
    for _ in 0..h.abs() {
        println!("{}", step);
    }
}
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(n)) = lines.by_ref().next() {
        let n = n.parse().expect("no a number");
        let grid : Vec<Vec<u8>> = lines.by_ref().take(n).map(|x| {
            x.expect("IO Error").into_bytes()
        }).collect();

        solve(grid);
    }
}
