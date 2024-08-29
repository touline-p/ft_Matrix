use super::{Field, Vector};
use std::ops::Index;

impl<K: Field, const SIZE: usize> Vector<K, SIZE> {
    pub fn new() -> Self {
        Vector::default()
    }

    pub fn size(&self) -> usize {
        SIZE
    }
}

impl<K: Field, const SIZE: usize> Default for Vector<K, SIZE> {
    fn default() -> Self {
        [K::default(); SIZE].into()
    }
}

impl<K: Field, const SIZE: usize> From<[K; SIZE]> for Vector<K, SIZE> {
    fn from(data: [K; SIZE]) -> Self {
        Vector {data}
    }
}

impl<K, const SIZE: usize> Index<usize> for Vector<K, SIZE> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    #[test]
    fn size_return_size() {
        const SIZE: usize = 10;
        let test = Vector::<i32, SIZE>::new();
        assert_eq!(SIZE, test.size())
    }

    #[test]
    fn new_default() {
        const SIZE: usize = 2;
        let test = Vector::<i32, SIZE>::new();
        let data = [0, 0];
        let to_compare = Vector { data };

        assert_eq!(test, to_compare)
    }
}
