#[derive(Copy, PartialEq, Eq, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            return None;
        }

        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }
        Some(Collatz { v: self.v })
    }
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Collatz {
            v: v * 2,
        }
    }
}

pub fn collatz(m: u64) -> usize {
    let mut n = m as u128;
    let mut steps: u64 = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        steps += 1;
    }
    steps as usize
}
