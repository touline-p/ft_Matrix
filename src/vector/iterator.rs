use super::Vector;

impl<K, const SIZE: usize> Vector<K, SIZE> {
    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut K> {
        self.data.iter_mut()
    }
}

impl<K, const SIZE: usize> IntoIterator for Vector<K, SIZE> {
    type Item = K;
    type IntoIter = std::array::IntoIter<K, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, K, const SIZE: usize> IntoIterator for &'a Vector<K, SIZE> {
    type Item = &'a K;
    type IntoIter = std::slice::Iter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<K, const SIZE: usize> FromIterator<K> for Vector<K, SIZE> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        Vector {
            data: std::array::from_fn(|_| iterator.next().unwrap()),
        }
    }
}



#[cfg(test)]
mod test {
    use crate::vector::Vector;

    #[test]
    fn iterate_on_default_vector() {
        const SIZE: usize = 10;
        let test = Vector::<i32, SIZE>::new();
        let mut i: usize = 0;

        for a in test {
            assert_eq!(a, 0);
            i += 1;
        }
        assert_eq!(i, SIZE);
    }
}
