use std::fmt::Debug;

struct Card {
    rows : [[u64; 5]; 5],
    marked : [[bool; 5]; 5],
}

impl Card {
    fn mark(&mut self, n : u64) {
        for i in 0..5 {
            for j in 0..5 {
                if self.rows[i][j] == n {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    fn winner(&self) -> bool {
        // rows
        'rows : for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    continue 'rows;
                }
            }

            return true
        }

        // cols
        'cols : for i in 0..5 {
            for j in 0..5 {
                if !self.marked[j][i] {
                    continue 'cols;
                }
            }

            return true
        }

        false
    }

    fn from_iter<'a, I>(it : &mut I) -> Option<Self>
    where
        I : Iterator<Item = &'a str>
    {
        match it.next() {
            Some(s) => assert!(s.is_empty()),
            None => return None,
        }

        fn f(s : &str) -> [u64; 5] {
            s
                .split_whitespace()
                .map(|x| x.parse().unwrap() : u64)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }

        Some(Self{
            rows : [
                f(it.next().unwrap()),
                f(it.next().unwrap()),
                f(it.next().unwrap()),
                f(it.next().unwrap()),
                f(it.next().unwrap()),
            ],
            marked : [[false; 5]; 5],
        })
    }

    fn unmarked<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        self
            .rows
            .iter()
            .enumerate()
            .flat_map(|(n, row)| {
                row
                    .iter()
                    .enumerate()
                    .map(move |(m, val)| {
                        (n, m, val)
                    })
            })
            .filter_map(|(n, m, val)| {
                (!self.marked[n][m]).then_some(*val)
            })
    }
}

pub fn run_a(input : &str) -> impl Debug {
    // row major


    let mut it = input.lines();
    let nums = it
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap() : u64)
        .collect() : Vec<_>;

    let mut cards = Vec::new();

    while let Some(card) = Card::from_iter(&mut it) {
        cards.push(card);
    }

    for n in nums {
        for card in cards.iter_mut() {
            card.mark(n);
            if card.winner() {
                return card.unmarked().sum::<u64>() * n
            }
        }
    }

    unreachable!()
}

pub fn run_b(input : &str) -> impl Debug {
    // row major


    let mut it = input.lines();
    let nums = it
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap() : u64)
        .collect() : Vec<_>;

    let mut cards = Vec::new();

    while let Some(card) = Card::from_iter(&mut it) {
        cards.push(card);
    }

    for n in nums {
        let l = cards.len();

        let mut it = cards.drain_filter(|card| {
            card.mark(n);
            card.winner()
        });

        match it.next() {
            Some(x) if l == 1 => {
                return x.unmarked().sum::<u64>() * n
            },
            _ => {},
        }

        it.count();
    }

    unreachable!()
}
