#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![Scalar::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![Scalar::zero();col];row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut a = Matrix(vec![vec![Scalar::zero();n];n]);
        for i in 0..n {
            a.0[i][i] = Scalar::one();
        }
        a
    }
}

use std::ops::{ Add, Sub, Mul, Div };

pub trait Scalar: Copy +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    PartialEq
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}
impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
