use super::{Matrix, Field};
use std::ops::{Sub, SubAssign};

impl<K: Field, const X: usize, const Y: usize> Sub for Matrix<K, X, Y> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| x - y).collect()
    }
}
impl<K: Field, const X: usize, const Y: usize> Sub for &Matrix<K, X, Y> {
    type Output = Matrix<K, X, Y>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| *x - *y).collect()
    }
}
impl<K: Field, const X: usize, const Y: usize> SubAssign for Matrix<K, X, Y> {
    fn sub_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(x, y)| *x -= y);
    }
}
