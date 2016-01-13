use std::prelude::v1::*;
use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    for line in stdin.lines() {
        let line = line.expect("IO Error");
        let line = line.trim();
        let time = &line[..line.len() - 2];
        let modif = &line[line.len() - 2 ..];
        let time : Vec<&str> = time.split(':').collect();
        let h = if modif == "AM" {
            if time[0] == "12" { 0u8 }else { time[0].parse::<u8>().unwrap() }
        } else {
            if time[0] == "12" { 12u8 } else { time[0].parse::<u8>().unwrap() + 12 }
        };
        println!("{:02}:{}:{}", h, time[1], time[2]);
    }
}
