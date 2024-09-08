use core::f32;
use crate::vector::Vector;
use std::ops::{Mul, Rem};

use super::Field;

pub fn lerp<V: Field + Mul<f32, Output = V>>(u: V, v: V, t: f32) -> V {
    u + (v - u) * t
}

pub fn angle_cos<K: Field, const SIZE: usize>(u: &Vector<K, SIZE>, v: &Vector<K, SIZE>) -> f64
        where K: Into<f64> {
    u.angle_cos(v)
}

pub fn cross_product<K>(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector::<K, 3>
        where K: Field {
    u.cross(v)
}

pub fn dot_product<K: Field, const SIZE: usize>(u: &Vector<K, SIZE>, v: &Vector<K, SIZE>) -> K {
    u.dot(v)
}

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

pub fn pgcd<K: Default + Rem<Output = K> + PartialEq + Clone>(mut a: K, mut b: K) -> K {
    loop {
        if b == K::default() {
            return a;
        }
        b = a.clone() % b;
        if a == K::default() {
            return b;
        }
        a = b.clone() % a;
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;
    use super::linear_combination;
    use super::lerp;

    #[test]
    fn lerp_basic() {
        assert_eq!(0.0, lerp(0., 1., 0.));
        assert_eq!(1.0, lerp(0., 1., 1.));
        assert_eq!(0.5, lerp(0., 1., 0.5));
        assert_eq!(27.3, lerp(21., 42., 0.3));
    }

    #[test]
    fn linear_combination_basic() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        assert_eq!(linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), [10., -2., 0.5].into());
        assert_eq!(linear_combination(&[v1, v2], &[10., -2.]), [10., 0., 230.].into());
    }
}
