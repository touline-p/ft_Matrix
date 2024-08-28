pub mod linear_combination;
pub mod lerp;
pub mod traits;
pub mod dot_product;

use crate::vector::{Vector, Field};

impl<K: Field, const SIZE: usize> Vector<K, SIZE> {
    pub fn add(&self, v: &Vector<K, SIZE>) -> Self {
        self + v
    }
    pub fn sub(&self, v: &Vector<K, SIZE>) -> Self {
        self - v
    }
    pub fn scl(&self, v: K) -> Self {
        self * v
    }
    pub fn add_assign(&mut self, v: &Vector<K, SIZE>) {
        self.iter_mut().zip(v).for_each(|(x, y)| *x += *y)
    }
    pub fn sub_assign(&mut self, v: &Vector<K, SIZE>) {
        self.iter_mut().zip(v).for_each(|(x, y)| *x -= *y)
    }
    pub fn scl_assign(&mut self, a: &K) {
        self.iter_mut().for_each(|x| *x *= *a)
    }
    pub fn add_scl_assign(&mut self, v: &Vector<K, SIZE>, coef: &K) {
        self.iter_mut().zip(v).for_each(|(x, y)| *x += *y * *coef)
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    #[test]
    fn add_vector() {
        let mut a = Vector::from([0,1,2]);
        let b = Vector::from([0,1,2]);
        assert_eq!(a.add(&b), [0,2,4].into());
        a.add_assign(&b);
        assert_eq!(a, [0,2,4].into());
        assert_eq!(a + b, [0,3,6].into());
    }

    #[test]
    fn sub_vector() {
        let mut a = Vector::from([0,1,2]);
        let b = Vector::from([0,1,2]);
        assert_eq!(a.sub(&b), [0,0,0].into());
        a.sub_assign(&b);
        assert_eq!(a, [0,0,0].into());
        assert_eq!(a - b, [0, -1, -2].into())
    }

    #[test]
    fn scl_vector() {
        let mut a = Vector::from([0,1,2]);
        assert_eq!(a.scl(3), [0,3,6].into());
        assert_eq!(&a * 3, [0,3,6].into());
        a.scl_assign(&3);
        assert_eq!(a, [0,3,6].into());
    }
}
