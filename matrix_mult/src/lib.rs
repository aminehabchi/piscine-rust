impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        if self.0.len() == 0 {
            return 0;
        }
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut a: Vec<T> = vec![];
        for i in 0..self.0.len() {
            a.push(self.0[i][n].clone());
        }
        a
    }
}

impl Mul for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, m: self) -> Self::Output {
        self::new()
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
