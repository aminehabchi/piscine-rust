#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers {
            numbers,
        }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() == 0 {
            return None;
        }
        return Some(self.numbers[self.numbers.len() - 1]);
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut a: Vec<u32> = self.numbers.to_vec();
        a.sort_unstable_by(|x, y| y.cmp(x));
        let mut i = 0;
        let mut ar = vec![];
        while i < 3 && i < a.len() {
            ar.push(a[i]);
            i += 1;
        }
        ar
    }
}
