pub mod basic;
pub mod format;
pub mod iterator;
pub mod ops;

use super::Field;

#[derive(PartialEq, Copy, Clone)]
pub struct Vector<K, const SIZE: usize> {
    data: [K; SIZE],
}
