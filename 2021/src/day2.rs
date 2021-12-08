use std::fmt::Debug;

pub fn run_a(input : &str) -> impl Debug {
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

pub fn run_b(input : &str) -> impl Debug {
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
