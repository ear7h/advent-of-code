

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

    (res >> 3) * 8 + (res & 7)
}

#[derive(Debug)]
struct Seats {
    data : Vec<u8>,
}

impl Seats {
    fn add(&mut self, id : u64) {
        assert!(id < 1025);

        self.data[(id / 8) as usize] |= 1 << (id % 8);
    }
}

fn main() {
    let input = stdin();
    let mut seats = Seats{
        data: std::iter::repeat(0).take(128).collect(),
    };
    for line in input.lock().lines() {
        seats.add(seat_id(line.unwrap().as_ref()));
    }

    for (idx, n) in seats.data.iter().enumerate() {
        if *n != 255  && *n != 0 {
            println!("{} {}", idx, n);
            println!("{}", (idx * 8)  as u32 + n.trailing_ones());
        }
    }
    println!("{:?}", seats);
}
