#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber {
    pub value: u32,
    pub digits: Vec<RomanDigit>,
}

use crate::RomanDigit::*;

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n == 0 || n >= 50000 {
            return RomanNumber {
                value: 0,
                digits: vec![Nulla],
            };
        }

        let roman: Vec<(u32, [RomanDigit; 2])> = vec![
            (1000, [M, Nulla]),
            (900, [C, M]),
            (500, [D, Nulla]),
            (400, [C, D]),
            (100, [C, Nulla]),
            (90, [X, C]),
            (50, [L, Nulla]),
            (40, [X, L]),
            (10, [X, Nulla]),
            (9, [I, X]),
            (5, [V, Nulla]),
            (4, [I, V]),
            (1, [I, Nulla]),
        ];

        let mut result = Vec::new();
        let mut remaining = n;

        for (value, digits) in roman.iter() {
            while remaining >= *value {
                remaining -= *value;
                result.push(digits[0]);
                if digits[1] != Nulla {
                    result.push(digits[1]);
                }
            }
        }

        RomanNumber {
            value: n,
            digits: result,
        }
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value >= 50000 {
            return None;
        }
        let next = RomanNumber::from(self.value);
        Some(next)
    }
}

