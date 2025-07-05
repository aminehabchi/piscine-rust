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
            v,
        }
    }
}

pub fn collatz(m: u64) -> usize {
    Collatz::new(m).count()
}
