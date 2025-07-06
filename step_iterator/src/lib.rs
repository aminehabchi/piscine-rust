pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
    pub is_started: bool,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            end,
            step,
            is_started: true,
        }
    }
}

impl<T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Clone> std::iter::Iterator
for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_started {
            self.is_started = false;

            return Some(self.beg.clone());
        }
        let a = self.beg.clone() + self.step.clone();
        if a > self.end {
            return None;
        }
        self.beg = a.clone();
        Some(a)
    }
}
