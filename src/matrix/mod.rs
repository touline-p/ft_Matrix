use crate::vector::Vector;
use crate::traits::Field;

pub mod basic;
pub mod ops;
pub mod iterator;

#[derive(Debug, PartialEq)]
pub struct Matrix<K, const SIZE_X: usize, const SIZE_Y: usize> {
    rows: [Vector<K, SIZE_X>; SIZE_Y],
}


