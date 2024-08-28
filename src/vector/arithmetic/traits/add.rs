use crate::vector::{Vector, traits::Field};
use std::ops::{Add, AddAssign};

impl<K: Field, const SIZE: usize> Add for Vector<K, SIZE> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| x + y).collect()
    }
}
impl<K: Field, const SIZE: usize> Add for &Vector<K, SIZE> {
    type Output = Vector<K, SIZE>;
    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().zip(rhs).map(|(x, y)| *x + *y).collect()
    }
}
impl<K: Field, const SIZE: usize> AddAssign for Vector<K, SIZE> {
    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs).for_each(|(x, y)| *x += y);
    }
}
