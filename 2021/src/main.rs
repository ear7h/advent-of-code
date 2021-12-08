// run with:
//  cargo +nightly run -- day (a | b) | wl-copy

#![feature(type_ascription,
           array_windows,
           bool_to_option,
           is_sorted,
           drain_filter)]


macro_rules! days {
    ($( $day:pat => $day_mod:ident),*,) => {

        $(
            mod $day_mod;
        )*

        fn main() {
            let mut it = std::env::args().skip(1);
            let day = it.next().unwrap().parse().unwrap() : u32;
            let version = it.next().unwrap().parse().unwrap() : char;
            let test = it.next().unwrap_or("false".into() ).parse().unwrap() : bool;

            eprintln!("{day} {version} {test}");

            let idx = (day - 1) * 2 + (if test { 1 } else { 0 });

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
                        let f = $day_mod::run_a;

                        println!("{:?}", f(INPUTS[idx as usize]))
                    },
                    ($day, 'b') => {
                        let f = $day_mod::run_b;

                        println!("{:?}", f(INPUTS[idx as usize]))
                    },
                )*
                _ => panic!("invalid day ({day}) or version ({version})"),
            }
        }
    }
}


days!{
    1 => day1,
    2 => day2,
    3 => day3,
    4 => day4,
}



