use std::prelude::v1::*;
use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    while let Some(Ok(n)) = lines.by_ref().next() {
        let n : usize = n.parse().expect("not a number");
        let mut i = 0;
        let mut sum1 = 0i32;
        let mut sum2 = 0i32;
        for line in lines.by_ref().take(n) {
            let line :String = line.expect("IO error");
            let nums1 = line.split_whitespace().map(|x| x.trim().parse::<i32>().expect("not a number"));
            let nums2 = line.split_whitespace().map(|x| x.trim().parse::<i32>().expect("not a number"));
            sum1 += nums1.skip(i).next().expect("too less");
            sum2 += nums2.rev().skip(i).next().expect("too less");
            i += 1;
        }
        println!("{}", (sum1 - sum2).abs());
    }
}
