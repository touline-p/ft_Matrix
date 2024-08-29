use crate::{vector::Vector, traits::Field};

impl<K: Field> Vector<K, 3> {
    pub fn cross(&self, other: &Vector<K, 3>) -> Vector<K, 3> {
        [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0]
        ].into()
    }
}

#[cfg(test)]
mod test {
    use super::Vector;
    #[test]
    fn test() {
        let u: Vector<f64, 3> = [0., 0., 1.].into();
        let v = [1., 0., 0.].into();
        assert_eq!(u.cross(&v), [0., 1., 0.].into());
        let u: Vector<f64, 3>  = [1., 2., 3.].into();
        let v = [4., 5., 6.].into();
        assert_eq!(u.cross(&v), [-3., 6., -3.].into());
        let u: Vector<f64, 3> = [4., 2., -3.].into();
        let v = [-2., -5., 16.].into();
        assert_eq!(u.cross(&v), [17., -58., -16.].into());
    }
}
