use super::{Vector, Field};


impl <K: Field, const SIZE: usize> Vector<K, SIZE>
        where K: Into<f64> {
    pub fn angle_cos(&self, other: &Self) -> f64 {
        self.dot(other).into() / (self.norm() * other.norm())
    }
}


#[cfg(test)]
mod test {
    use super::Vector;
    #[test]
    fn vector_angle_droit() {
        let u: Vector<f64, 2> = [0., 1.].into();
        let v = [1., 0.].into();
        assert_eq!(0., u.angle_cos(&v));
    }

    #[test]
    fn vector_angle_plat() {
        let u: Vector<f64, 2> = [0., 1.].into();
        let v = [0., -1.].into();
        assert_eq!(1., u.angle_cos(&u));
        assert_eq!(-1., u.angle_cos(&v));
    }
    #[test]
    fn test_from_subject() {
        let u: Vector<f64, 3> = [1., 2., 3.].into();
        let v = [4., 5., 6.].into();
        assert_eq!(u.angle_cos(&v), 0.9746318461970762);
    }
}
