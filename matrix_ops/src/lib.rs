impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, m2: Self) -> Self::Output {
        if self.0.len() != m2.0.len() {
            return None;
        }
        let mut m: Matrix<T> = Matrix::zero(self.0.len(), m2.0[0].len());
        for i in 0..m2.0.len() {
            if self.0[i].len() != m2.0[i].len() {
                return None;
            }
            for j in 0..self.0[i].len() {
                m.0[i][j] = self.0[i][j] + m2.0[i][j];
            }
        }
        Some(m)
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, m2: Self) -> Self::Output {
        if self.0.len() != m2.0.len() {
            return None;
        }
        let mut m: Matrix<T> = Matrix::zero(self.0.len(), m2.0[0].len());
        for i in 0..m2.0.len() {
            if self.0[i].len() != m2.0[i].len() {
                return None;
            }
            for j in 0..self.0[i].len() {
                m.0[i][j] = self.0[i][j] - m2.0[i][j];
            }
        }
        Some(m)
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
