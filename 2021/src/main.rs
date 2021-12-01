#![feature(type_ascription, array_windows, bool_to_option, is_sorted)]

use std::fmt::Debug;
use std::iter::IntoIterator;
use std::ops::Not;

fn main() {
    let mut it = std::env::args().skip(1);
    let day = it.next().unwrap().parse().unwrap() : u32;
    let version = it.next().unwrap().parse().unwrap() : char;
    let test = it.next().unwrap_or("false".into() ).parse().unwrap() : bool;

    println!("{day} {version} {test}");

    let idx = (day - 1) * 2 + (if test { 1 } else { 0 });

    match (day, version) {
        (1, 'a') => println!("{:?}", run_1a(INPUTS[idx as usize])),
        (1, 'b') => println!("{:?}", run_1b(INPUTS[idx as usize])),
        _ => panic!("invalid day ({day}) or version ({version})"),
    }
}

const INPUTS : &'static [&'static str] = &[
    include_str!("inputs/1.txt"),
    include_str!("inputs/1-test.txt"),
];


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

