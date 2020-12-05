

use std::{
    io::stdin,
    io::prelude::*,
};

fn seat_id(s : &str) -> u64 {
    let mut res = 0;
    for v in s.chars() {
        res = res << 1;
        res |= match v {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        };
    }

    //println!("{} {}", res >> 3, res & 7);
    (res >> 3) * 8 + (res & 7)
}

fn main() {
    let input = stdin();
    let mut max = 0;
    for line in input.lock().lines() {
        max = max.max(seat_id(line.unwrap().as_ref()));
    }
    println!("{}", max);
}
