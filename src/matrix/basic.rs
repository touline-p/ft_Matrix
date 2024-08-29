use std::ops::Index;

use crate::vector::Vector;

use super::{Matrix, Field};

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
    where K: Field {
    pub fn new() -> Self {
        Matrix::default()
    }

    pub fn shape() -> (usize, usize) {
        (X, Y)
    }

    pub fn is_square() -> bool {
        X == Y
    }
}

impl<K: Field, const X: usize, const Y: usize> From<[Vector<K, X>; Y]> for Matrix<K, X, Y> {
    fn from(rows: [Vector<K, X>; Y]) -> Self {
        Matrix { rows }
    }
}

impl<K: Field + Copy, const X: usize, const Y: usize> Default for Matrix<K, X, Y> {
    fn default() -> Self {
        Matrix { rows: [Vector::<K, X>::default(); Y]}
    }
}

impl<K: Field, const X: usize, const Y: usize> Index<usize> for Matrix<K, X, Y> {
    type Output = Vector<K, X>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
