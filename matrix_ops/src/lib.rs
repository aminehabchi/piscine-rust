use matrix::*;

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
