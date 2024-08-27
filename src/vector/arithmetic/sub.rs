use crate::vector::{Vector, traits::Field};
use std::ops::{Sub, SubAssign};

impl<K: Field, const SIZE: usize> Sub for Vector<K, SIZE> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| x - y).collect()
    }
}
impl<K: Field, const SIZE: usize> Sub for &Vector<K, SIZE> {
    type Output = Vector<K, SIZE>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| *x - *y).collect()
    }
}
impl<K: Field, const SIZE: usize> SubAssign for Vector<K, SIZE> {
    fn sub_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(x, y)| *x -= y);
    }
}


