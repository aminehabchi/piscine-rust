use crate::RomanDigit::*;

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
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}


impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 || n >= 50000 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        // Always 2 elements; pad with Nulla if only one digit
        let roman: Vec<(u32, [RomanDigit; 2])> = vec![
            (1000, [M, Nulla]),
            (900,  [C, M]),
            (500,  [D, Nulla]),
            (400,  [C, D]),
            (100,  [C, Nulla]),
            (90,   [X, C]),
            (50,   [L, Nulla]),
            (40,   [X, L]),
            (10,   [X, Nulla]),
            (9,    [I, X]),
            (5,    [V, Nulla]),
            (4,    [I, V]),
            (1,    [I, Nulla]),
        ];

        let mut result = Vec::new();

        for (value, digits) in roman.iter() {
            while n >= *value {
                n -= *value;
                result.push(digits[0]);
                if digits[1] != Nulla {
                    result.push(digits[1]);
                }
            }
        }

        RomanNumber(result)
    }
}