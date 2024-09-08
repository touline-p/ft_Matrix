use crate::vector::Vector;
use crate::traits::Field;

pub mod basic;
pub mod ops;
pub mod iterator;
pub mod row_echelon_form;
pub mod inverse;
pub mod transpose;
pub mod rank;
pub mod projection;

#[derive(PartialEq, Clone)]
pub struct Matrix<K, const SIZE_X: usize, const SIZE_Y: usize> {
    rows: [Vector<K, SIZE_X>; SIZE_Y],
}

pub type MatrixResult<T> = Result<T, MatrixError>;

#[derive(Debug)]
pub enum MatrixError {
    BadRowEchelonForm
}

