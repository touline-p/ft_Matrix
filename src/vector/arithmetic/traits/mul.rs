use crate::vector::{Vector, traits::Field};
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
