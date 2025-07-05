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
            return Some(*self); // include starting value
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
    Collatz::new(m).count()
}