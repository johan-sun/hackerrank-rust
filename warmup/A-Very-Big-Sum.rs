use std::io::prelude::*;
use std::prelude::v1::*;


fn add( mut num1 : Vec<u8>, num2: &str ) -> Vec<u8> {
    if num2.len() > num1.len() {
        num1.resize(num2.len(), b'0');
    }

    let mut carry = 0;
    for ( i, num ) in num2.as_bytes().iter().rev().enumerate() {
        let a = (num - b'0') + (num1[i] - b'0') + carry;
        num1[i] = a % 10 + b'0';
        carry = a / 10;
    }

    for i in num2.len()..num1.len() {
        if carry == 0 {
            break;
        }
        let a = ( num1[i] - b'0' ) + carry;
        num1[i] = a % 10;
        carry = a / 10;
    }

    if carry > 0 {
        num1.push(b'1');
    }

    num1
}

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    for line in stdin.lines().enumerate().filter(|&(ln, _)| ln & 1 == 1).map(|(_, line)| line.expect("IO error")) {
        let mut rst = line.split_whitespace().fold(Vec::new() as Vec<u8>, add);
        rst.reverse();
        println!("{}", unsafe{ String::from_utf8_unchecked(rst)});
    }
}
