use std::prelude::v1::*;
use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(n)) = lines.next() {
        let n : usize = n.parse().expect("not a number");
        for i in 1..n+1 {
            for &b in [b' '].iter().cycle().take(n-i).chain([b'#'].iter().cycle().take(i)) {
                print!("{}", b as char);
            }
            println!("");
        }
    }
}
