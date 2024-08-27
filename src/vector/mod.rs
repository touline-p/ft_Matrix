pub mod arithmetic;
pub mod basic;
pub mod traits;
pub mod format;
pub mod iterator;

use traits::Field;

#[derive(PartialEq)]
pub struct Vector<K, const SIZE: usize> {
    data: [K; SIZE],
}
