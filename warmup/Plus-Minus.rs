use std::prelude::v1::*;
use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let  mut lines = stdin.lines();
    while let Some(Ok(n)) = lines.by_ref().next() {
        let n : i32 = n.parse().expect("no a number");
        let mut npos = 0;
        let mut nneg = 0;
        let mut nzero = 0;
        for num in lines.by_ref().next().expect("EOF error").expect("IO error").split_whitespace().map(|x| x.trim().parse::<i32>().expect("not a number")) {
            if num < 0 {
                nneg += 1;
            }else if num > 0 {
                npos += 1;
            }else {
                nzero += 1;
            }
        }

        println!("{:.6}", npos as f64 / n as f64);
        println!("{:.6}", nneg as f64 / n as f64);
        println!("{:.6}", nzero as f64 / n as f64);
    }
}
