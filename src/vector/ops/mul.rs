use crate::{matrix::Matrix, traits::Field, vector::Vector};
use std::ops::{Mul, MulAssign};

impl<K: Field, const SIZE: usize> Mul<K> for Vector<K, SIZE> {
    type Output = Vector<K, SIZE>;

    fn mul(self, rhs: K) -> Self::Output {
        self.iter().map(|x| *x * rhs).collect()
    }
}
impl<K: Field, const SIZE: usize> Mul<K> for &Vector<K, SIZE> {
    type Output = Vector<K, SIZE>;

    fn mul(self, rhs: K) -> Self::Output {
        self.iter().map(|x| *x * rhs).collect()
    }
}
impl<K: Field, const SIZE: usize> MulAssign<K> for Vector<K, SIZE> {
    fn mul_assign(&mut self, rhs: K) {
        self.iter_mut().for_each(|x| *x *= rhs)
    }
}

impl<K: Field, const SIZE: usize> Mul<Matrix<K, SIZE, SIZE>> for Vector<K, SIZE> {
    type Output = Vector<K, SIZE>;

    fn mul(self, rhs: Matrix<K, SIZE, SIZE>) -> Self::Output {
        rhs.into_iter().map(|row| row.dot(&self)).collect()
    }
}
