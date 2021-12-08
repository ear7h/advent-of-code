use std::fmt::Debug;

pub fn run_a(input : &str) -> impl Debug {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {
                    match c {
                        '0' => -1,
                        '1' => 1,
                        _ => unreachable!(),
                    }
                })
                .collect() : Vec<_>
        })
        .reduce(|mut l, mut r| {
            l
                .drain(0..)
                .zip(r.drain(0..))
                .map(|(l, r)| l + r)
                .collect() : Vec<_>
        })
        .unwrap()
        .drain(0..)
        .map(|n| {
            use std::cmp::Ordering::*;

            match n.cmp(&0) {
                Less => 0,
                Greater => 1,
                Equal => unreachable!(),
            }
        })
        .try_fold((0, 0), |(a1, a2), el| {
            Some((
                a1 << 1 | el,
                a2 << 1 | (el ^ 1),
            ))
        })
        .map(|(a1, a2)| {
            a1 * a2
        })
        .unwrap()
}

pub fn run_b(input : &str) -> impl Debug {
    let bits = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {
                    match c {
                        '0' => -1,
                        '1' => 1,
                        _ => unreachable!(),
                    }
                })
                .collect() : Vec<_>
        })
        .collect() : Vec<_>;

    let mut buf = bits.clone();

    for i in 0.. {
        use std::cmp::Ordering::*;

        if buf.len() == 1 {
            break
        }

        let n = buf
            .iter()
            .fold(0, |acc, el| acc + el[i]);

        let n = match n.cmp(&0) {
            Less => -1,
            Greater | Equal => 1,
        };


        buf.drain_filter(|el| el[i] != n).count();
    }

    let oxy = buf[0].iter().fold(0, |acc, el| acc << 1 | ((el + 1) / 2));

    let mut buf = bits;

    for i in 0.. {
        use std::cmp::Ordering::*;

        if buf.len()  == 1 {
            break
        }

        let n = buf
            .iter()
            .fold(0, |acc, el| acc + el[i]);

        let n = match n.cmp(&0) {
            Less => 1,
            Greater | Equal => -1,
        };


        buf.drain_filter(|el| el[i] != n).count();
    }

    let co2 = buf[0].iter().fold(0, |acc, el| acc << 1 | ((el + 1) / 2));

    oxy * co2
}

