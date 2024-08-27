use super::{Vector, Field};

pub fn linear_combination<K: Field, const SIZE: usize, const NB_OF_VEC: usize>(
        u: &[Vector<K, SIZE>; NB_OF_VEC],
        coefs: &[K; NB_OF_VEC]
    ) -> Vector<K, SIZE> {
    let mut return_value = Vector::<K,SIZE>::new();
    for (vector, coef) in u.iter().zip(coefs) {
        return_value.add_scl_assign(vector, coef);
    }
    return_value
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;
    use super::linear_combination;
    #[test]
    fn linear_combination_basic() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert_eq!(linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), Vector::from([10., -2., 0.5]));
        assert_eq!(linear_combination(&[v1, v2], &[10., -2.]), [10., 0., 230.].into());
    }
}
