use std::prelude::v1::*;
use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();//get stdin
    let stdin = stdin.lock(); //it is faster
    for line in stdin.lines().enumerate().filter(|&(li, _) | li & 1 == 1).map(|(_, line)| line.expect("io error")) {
        println!("{}", line.split_whitespace().map(|s| s.trim().parse::<i32>().expect("not a number")).fold(0, | acc, n | acc + n));
    }
}
