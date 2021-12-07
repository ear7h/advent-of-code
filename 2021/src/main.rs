// run with:
//  cargo +nightly run -- day (a | b) | wl-copy

#![feature(type_ascription,
           array_windows,
           bool_to_option,
           is_sorted,
           drain_filter,
           concat_idents)]

use std::fmt::Debug;
use std::iter::IntoIterator;
use std::ops::Not;

fn main() {
    let mut it = std::env::args().skip(1);
    let day = it.next().unwrap().parse().unwrap() : u32;
    let version = it.next().unwrap().parse().unwrap() : char;
    let test = it.next().unwrap_or("false".into() ).parse().unwrap() : bool;

    eprintln!("{day} {version} {test}");

    let idx = (day - 1) * 2 + (if test { 1 } else { 0 });

    /*
    let run = |s| {
        Box::new(match (day, version) {
            (1, 'a') => run_1a(s),
            (1, 'b') => run_1b(s),
            (2, 'a') => run_2a(s),
            (2, 'b') => run_2b(s),
            (3, 'a') => run_3a(s),
            (3, 'b') => run_3b(s),
            _ => panic!("invalid day ({day}) or version ({version})"),
        })
    };
    */

    macro_rules! days {
        ($( $day:pat => $run:ident),*,) => {
            const INPUTS : &'static [&'static str] = &[
                $(
                    include_str!{
                        concat!{
                            "inputs/",
                            stringify!($day),
                            ".txt"
                        }
                    },
                    include_str!{
                        concat!{
                            "inputs/",
                            stringify!($day),
                            "-test.txt"
                        }
                    },
                )*
            ];


            match (day, version) {
                $(
                    ($day, 'a') => {
                        let f = concat_idents!($run, a);

                        println!("{:?}", f(INPUTS[idx as usize]))
                    },
                    ($day, 'b') => {
                        let f = concat_idents!($run, b);

                        println!("{:?}", f(INPUTS[idx as usize]))
                    },
                )*
                _ => panic!("invalid day ({day}) or version ({version})"),
            }
        }
    }

    days!{
        1 => run_1,
        2 => run_2,
        3 => run_3,
    }
}

trait IteratorPlus : Iterator + Sized {
    fn array_windows<const N: usize>(mut self)
        -> ArrayWindows<Self::Item, Self, N>
    {
        let mut v = Vec::new();

        for _ in 0..N {
            match self.next() {
                None => {
                    return ArrayWindows {
                        it : self,
                        started : true,
                        cache : None,
                    }
                },
                Some(x) => v.push(x),
            }
        }

        let cache = if let Ok(x) = v.try_into() {
            Some(x)
        } else {
            unreachable!()
        };

        ArrayWindows{
            it : self,
            started : false,
            cache
        }
    }
}

impl<I> IteratorPlus for I
where
    I : Iterator
{}

struct ArrayWindows<T, I, const N : usize> {
    it : I,
    started : bool,
    cache : Option<[T; N]>,
}

impl<T, I, const N : usize> Iterator for ArrayWindows<T, I, N>
where
    T : Clone,
    I : Iterator<Item = T>,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {

        if !self.started {
            self.started = true;
            return self.cache.clone()
        }

        match (self.it.next(), self.cache.as_mut()) {
            (Some(x), Some(cache)) => {

                cache.rotate_left(1);
                cache[N - 1] = x;
                Some(cache.clone())
            }
            (None, _) | (_, None) => None,
        }
    }
}


fn run_1a(input : &str) -> impl Debug {
    input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .array_windows::<2>()
        .map(<_ as IntoIterator>::into_iter)
        .map(<_ as Iterator>::rev)
        .map(<_ as Iterator>::is_sorted)
        .map(Not::not)
        .filter(Clone::clone)
        .count()
}

fn run_1b(input : &str) -> impl Debug {
    input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .array_windows::<3>()
        .map(<_ as IntoIterator>::into_iter)
        .map(<_ as Iterator>::sum::<u32>)
        .array_windows::<2>()
        .map(<_ as IntoIterator>::into_iter)
        .map(<_ as Iterator>::rev)
        .map(<_ as Iterator>::is_sorted)
        .map(Not::not)
        .filter(Clone::clone)
        .count()
}

fn run_2a(input : &str) -> impl Debug {
    let (h, d) = input
        .lines()
        .map(|s| {

            let (dir, n) = s.split_once(' ').unwrap();
            (dir, n.parse().unwrap() : u32)
        })
        .fold((0, 0), |(h, d), (dir, n)| {
            match dir {
                "forward" => (h + n, d),
                "down" => (h, d + n),
                "up" => (h, d - n),
                _ => (h, d),
            }
        });
    h * d
}

fn run_2b(input : &str) -> impl Debug {
    let (h, d, _) = input
        .lines()
        .map(|s| {

            let (dir, n) = s.split_once(' ').unwrap();
            (dir, n.parse().unwrap() : u32)
        })
        .fold((0, 0, 0), |(h, d, a), (dir, n)| {
            match dir {
                "forward" => (h + n, d + a * n, a),
                "down" => (h, d, a + n),
                "up" => (h, d, a - n),
                _ => (h, d, a),
            }
        });
    h * d
}

fn run_3a(input : &str) -> impl Debug {
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

fn run_3b(input : &str) -> impl Debug {
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
