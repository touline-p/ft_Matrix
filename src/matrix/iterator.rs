use std::iter::FusedIterator;

use super::{Matrix, Field};
use crate::vector::Vector;

struct MatrixIter
<'a, K, const X: usize,
const Y: usize,
const STEP_X: usize,
const STEP_Y: usize> {
    matrix: &'a Matrix<K, X, Y>,
    col: usize,
    row: usize,
}

struct MatrixColIter
<'a, K, const X: usize,
const Y: usize> {
    matrix: &'a Matrix<K, X, Y>,
    col: usize,
    row: usize,
}

impl<
    'a, K: Field, const X: usize, const Y: usize,
> Iterator for MatrixColIter<'a, K, X, Y> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < Y {
            let option = Some(&self.matrix[self.row][self.col]);
            self.row += 1;
            option
        } else {
            None
        }
    }
}

impl<
    'a, K: Field, const X: usize, const Y: usize,
    const STEP_X: usize, const STEP_Y: usize
> Iterator for MatrixIter<'a, K, X, Y, STEP_X, STEP_Y> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < X && self.row < Y {
            let option = Some(&self.matrix[self.row][self.col]);
            self.col += STEP_X;
            self.row += STEP_Y;
            option
        } else {
            None
        }
    }
}

//impl<
    //'a, K: Field, const X: usize, const Y: usize,
    //const STEP_X: usize, const STEP_Y: usize>
    //MatrixIter<'a, K, X, Y, STEP_X, STEP_Y> {
        //pub fn new(matrix, row: usize, col: usize) {
            //MatrixIter {
//
            //}
        //}
//}
//
impl<
    'a, K: Field, const X: usize, const Y: usize,
    const STEP_X: usize, const STEP_Y: usize
> FusedIterator for MatrixIter<'a, K, X, Y, STEP_X, STEP_Y> {}

impl<'a, K: Field, const X: usize, const Y: usize>
    Matrix<K, X, Y> {

    pub fn iter(&self) -> impl Iterator<Item = &Vector<K, X>> {
        self.rows.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vector<K, X>> {
        self.rows.iter_mut()
    }
    pub fn matrix_iter<const STEP_X: usize, const STEP_Y: usize>(&'a self, col: usize, row: usize) -> impl Iterator<Item = &K> {
        MatrixIter::<K, X, Y, STEP_X, STEP_Y> {
            matrix: self,
            row,
            col
        }
    }
    pub fn matrix_col_iter(&'a self, col: usize) -> impl Iterator<Item = &K> {
        MatrixColIter::<K, X, Y> {
            matrix: self,
            row: 0,
            col
        }
    }
}

impl<K, const X: usize, const Y: usize> IntoIterator for Matrix<K, X, Y> {
    type Item = Vector<K, X>;
    type IntoIter = std::array::IntoIter<Vector<K, X>, Y>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a, K, const X: usize, const Y: usize> IntoIterator for &'a Matrix<K, X, Y> {
    type Item = &'a Vector<K, X>;
    type IntoIter = std::slice::Iter<'a, Vector<K, X>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

impl<K, const X: usize, const Y: usize> FromIterator<Vector<K, X>> for Matrix<K, X, Y> {
    fn from_iter<T: IntoIterator<Item = Vector<K, X>>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        Matrix {
            rows: std::array::from_fn(|_| iterator.next().unwrap()),
        }

    }
}

//impl<'a, K, const X: usize, const Y: usize> FromIterator<&'a Vector<K, X>> for Matrix<K, X, Y> {
    //fn from_iter<T: IntoIterator<Item = &'a Vector<K, X>>>(iter: T) -> Self {
        //let mut iterator = iter.into_iter();
        //Matrix {
            //rows: std::array::from_fn(|_| iterator.next().unwrap().clone()),
        //}
//
    //}
//}
