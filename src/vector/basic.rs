use super::{Field, Vector};
use std::ops::{Index, IndexMut};

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


impl<K, const SIZE: usize> IndexMut<usize> for Vector<K, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<K, const SIZE: usize> Index<usize> for Vector<K, SIZE> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K, const SIZE: usize> Vector<K, SIZE>
        where K:Field {
    pub fn nulify_index(&mut self, index: usize, pivot_v: &Self, pivot: K) {
        let coef_self = self[index];
        let pivot_coef = pivot_v[index];
        self.iter_mut().zip(pivot_v.iter())
            .for_each(|(s, p)| *s = (pivot_coef * *s - coef_self * *p) / pivot);
    }

    pub fn nulify_index_inv(&mut self, index: usize, pivot_v: &Self, pivot: K, identity_row: &mut Self, pivot_v_i: &Self) {
        let coef_self = self[index].clone();
        let pivot_coef = pivot_v[index];
        self.iter_mut().zip(pivot_v.iter())
            .for_each(|(s, p)| *s = (pivot_coef * *s - coef_self * *p) / pivot);
        identity_row.iter_mut().zip(pivot_v_i.iter())
            .for_each(|(i, p)| *i = (pivot_coef * *i - coef_self * *p) / pivot);

    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use crate::vector::{Vector, Field, };

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

    fn test_pivot<K: Field + Debug + PartialEq, const SIZE: usize>(test: [K; SIZE], pivot: [K; SIZE], result: [K; SIZE]) {
        let pivot : Vector::<K, SIZE> = pivot.into();
        let mut test: Vector<K, SIZE> = test.into();
        test.nulify_index(0, &pivot, K::unity());
        let result: Vector<K, SIZE>  = result.into();
        assert_eq!(result, test);
    }

    #[test]
    fn pivot() {
        // test pivot takes a vector to test a pivot and a result
        test_pivot([5,2,4],[1,2,4],[0,-8,-16]);
        test_pivot([-1,0,2,-3],[2,-3,1,4],[0,-3,5,-2]);
    }
}
