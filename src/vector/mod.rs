pub mod arithmetic;
pub mod basic;
pub mod field;
pub mod format;
pub mod iterator;

use field::Field;

#[derive(PartialEq)]
pub struct Vector<K, const SIZE: usize> {
    data: [K; SIZE],
}
