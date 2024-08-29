use super::{Matrix, Field};
use std::ops::{Add, AddAssign};

impl<K: Field, const X: usize, const Y: usize> Add for Matrix<K, X, Y> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| x + y).collect()
    }
}
impl<K: Field, const X: usize, const Y: usize> Add for &Matrix<K, X, Y> {
    type Output = Matrix<K, X, Y>;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| *x + *y).collect()
    }
}
impl<K: Field, const X: usize, const Y: usize> AddAssign for Matrix<K, X, Y> {
    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(x, y)| *x += y);
    }
}
