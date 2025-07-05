#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Collatz {
    pub v: u64,
    started: bool,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(*self);
        }

        if self.v <= 1 {
            return None;
        }

        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }

        Some(self.clone())
    }
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Collatz { v, started: false }
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