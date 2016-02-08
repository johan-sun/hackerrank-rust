use std::prelude::v1::*;
use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t : i32 = lines.next().expect("Need line of T").expect("Should no io error").trim().parse().expect("Should be i32");
    for _ in 0..t {
        let line = lines.next().expect("Need line of N K").expect("Should no io error");
        let mut nk = line.trim().split_whitespace().map(|s|s.parse::<usize>().expect("Should be usize"));
        let n = nk.next().expect("N");
        let k = nk.next().expect("K");
        let line = lines.next().expect("Need student arrival time").expect("Should no io error");
        println!("{}", 
            if line.trim().split_whitespace().map(|s|s.parse::<i32>().expect("Should be i32"))
                .take(n)
                .filter(|&t| t <= 0)
                .count() < k { "YES" } else { "NO" });
    }
}
