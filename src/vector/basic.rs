use super::{Field, Vector};

impl<K: Field, const SIZE: usize> Vector<K, SIZE> {
    pub fn new() -> Self {
        Vector {
            data: [K::default(); SIZE],
        }
    }

    pub fn size(&self) -> usize {
        SIZE
    }
}

impl<K: Field, const SIZE: usize> From<[K; SIZE]> for Vector<K, SIZE> {
    fn from(data: [K; SIZE]) -> Self {
        Vector {data}
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
