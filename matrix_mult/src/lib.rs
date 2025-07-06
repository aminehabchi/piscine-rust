use std::ops::Mul;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() { 0 } else { self.0[0].len() }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0
            .iter()
            .map(|row| row[n])
            .collect()
    }
}

impl<
    T: std::ops::Mul + Copy + Default + std::ops::Add<<T as std::ops::Mul>::Output, Output = T>
> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();

        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::default();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k] * rhs.0[k][j];
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
